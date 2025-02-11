pub mod commands;
pub mod fuzzer;
pub mod state;
pub mod ui;

use commands::{get_by_name, CmdGroup};
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    commands: Vec<String>,
    fuzzed_commands: Vec<(String, f32)>,
    option_selection: Vec<String>,
    search_input: String,
    selected_cmd: usize,
    render_mode: RenderMode,
    selected_option: usize,
    cmd_input: String,
    tile: Tile,
    message_content: String,
}

#[derive(Default)]
enum RenderMode {
    #[default]
    Normal,
    Input,
    OptionSelect,
    Message,
}

#[derive(Default)]
struct Tile {
    pane_manifest: PaneManifest,
    tab_info: Vec<TabInfo>,
    focused_tab: TabInfo,
    focused_pane: PaneInfo,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
            PermissionType::OpenTerminalsOrPlugins,
        ]);

        self.commands = commands::get_list();

        subscribe(&[
            EventType::Key,
            EventType::ModeUpdate,
            EventType::TabUpdate,
            EventType::PaneUpdate,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;

        match event {
            Event::PaneUpdate(pane_manifest) => {
                self.tile.pane_manifest = pane_manifest.clone();
                self.update_tile();
            }
            Event::TabUpdate(tab_info) => {
                self.tile.tab_info = tab_info.clone();
                self.update_tile();
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Esc => match self.render_mode {
                    RenderMode::Normal => {
                        close_focus();
                    }
                    _ => {
                        self.reset();
                        should_render = true;
                    }
                },
                BareKey::Char(char) if char.is_alphanumeric() || char.is_whitespace() => {
                    match self.render_mode {
                        RenderMode::Normal => {
                            self.search_input.push(char);
                            self.fuzz();
                        }
                        RenderMode::Input => {
                            self.cmd_input.push(char);
                        }
                        _ => {}
                    }
                    should_render = true;
                }
                BareKey::Backspace => {
                    match self.render_mode {
                        RenderMode::Normal => {
                            self.search_input.pop();
                            self.fuzz();
                        }
                        RenderMode::Input => {
                            self.cmd_input.pop();
                        }
                        _ => {}
                    }
                    should_render = true;
                }
                BareKey::Down => match self.render_mode {
                    RenderMode::Normal | RenderMode::OptionSelect => {
                        self.select_down();
                        should_render = true;
                    }
                    _ => {}
                },
                BareKey::Up => match self.render_mode {
                    RenderMode::Normal | RenderMode::OptionSelect => {
                        self.select_up();
                        should_render = true;
                    }
                    _ => {}
                },
                BareKey::Enter => {
                    if !self.fuzzed_commands.is_empty() {
                        if let Some(current_cmd) =
                            get_by_name(self.fuzzed_commands[self.selected_cmd].0.as_str())
                        {
                            let group = current_cmd.group.clone();
                            match self.render_mode {
                                RenderMode::Normal => {
                                    match current_cmd.name {
                                        "Quit" => quit_zellij(),
                                        "TogglePaneFrames" => toggle_pane_frames(),
                                        "BreakPaneLeft" => {}
                                        "ScrollUp" => scroll_up_in_pane_id(PaneId::Terminal(
                                            self.tile.focused_pane.id,
                                        )),
                                        "ScrollUpAt" => scroll_up_in_pane_id(PaneId::Terminal(
                                            // TODO no matching func in shim
                                            self.tile.focused_pane.id,
                                        )),
                                        "ScrollDown" => scroll_down_in_pane_id(PaneId::Terminal(
                                            self.tile.focused_pane.id,
                                        )),
                                        "ScrollDownAt" => scroll_down_in_pane_id(PaneId::Terminal(
                                            // TODO no matching func in shim
                                            self.tile.focused_pane.id,
                                        )),
                                        "PageScrollDown" => page_scroll_down_in_pane_id(
                                            PaneId::Terminal(self.tile.focused_pane.id),
                                        ),
                                        "PageScrollUp" => page_scroll_up_in_pane_id(
                                            PaneId::Terminal(self.tile.focused_pane.id),
                                        ),
                                        "Write" => self.render_mode = RenderMode::Input,
                                        "SwitchToMode" => {
                                            if let Some(opts) = current_cmd.options {
                                                let o = opts.iter().map(|&s| s.into()).collect();
                                                self.option_selection = o;
                                                self.render_mode = RenderMode::OptionSelect
                                            } else {
                                                self.error_msg(
                                                    "failed to fetch options".to_string(),
                                                );
                                            }
                                        }
                                        "FocusNextPane" => {
                                            focus_next_pane();
                                        }
                                        "FocusPreviousPane" => {
                                            focus_previous_pane();
                                        }
                                        "SwitchFocus" => {
                                            self.error_msg("Not implemented".to_string());
                                        }
                                        "RenameSession" => self.render_mode = RenderMode::Input,
                                        "Resize" => {
                                            switch_to_input_mode(&InputMode::Resize);
                                        }
                                        "MoveFocus" => {
                                            if let Some(opts) = current_cmd.options {
                                                let o = opts.iter().map(|&s| s.into()).collect();
                                                self.option_selection = o;
                                                self.render_mode = RenderMode::OptionSelect
                                            } else {
                                                self.error_msg(
                                                    "failed to fetch options".to_string(),
                                                );
                                            }
                                        }
                                        "NewPane" => {
                                            open_terminal(Path::new(""));
                                        }

                                        &_ => {}
                                    }
                                    if matches!(group, CmdGroup::Direct) {
                                        close_focus()
                                    }
                                }
                                RenderMode::Input => match current_cmd.name {
                                    "RenameSession" => {
                                        rename_session(self.cmd_input.as_str());
                                        self.reset();
                                    }
                                    "Write" => {
                                        let bytes = self.cmd_input.clone().into_bytes();
                                        let pane_id = PaneId::Terminal(self.tile.focused_pane.id);

                                        write_to_pane_id(bytes, pane_id)
                                    }
                                    &_ => {}
                                },
                                RenderMode::Message => {
                                    self.reset();
                                }
                                RenderMode::OptionSelect => {
                                    match current_cmd.name {
                                        "SwitchToMode" => {
                                            // TODO should this reset state?
                                            let mode = &self.option_selection[self.selected_option];
                                            if let Ok(m) = InputMode::from_str(mode) {
                                                switch_to_input_mode(&m);
                                            } else {
                                                self.error_msg("input mode not found".to_string());
                                            }
                                        }
                                        "MoveFocus" => {
                                            let dir = &self.option_selection[self.selected_option];
                                            if let Ok(d) = Direction::from_str(dir) {
                                                move_focus_or_tab(d);
                                            } else {
                                                self.error_msg(
                                                    "Move direction not found".to_string(),
                                                );
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        } else {
                            self.error_msg("failed to fetch cmd".to_string());
                        }
                        should_render = true;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        should_render
    }

    fn pipe(&mut self, _pipe_message: PipeMessage) -> bool {
        true
    }

    fn render(&mut self, rows: usize, cols: usize) {
        match self.render_mode {
            RenderMode::Normal => ui::normal_render(
                &self.search_input,
                self.selected_cmd,
                self.fuzzed_commands.clone(),
                &self.commands,
                rows,
                cols,
            ),
            RenderMode::Input => ui::input_render(
                self.selected_cmd,
                self.cmd_input.clone(),
                self.fuzzed_commands.clone(),
            ),
            RenderMode::Message => ui::message_render(self.message_content.clone()),
            RenderMode::OptionSelect => ui::select_render(
                self.option_selection.clone(),
                self.selected_option,
                rows,
                cols,
            ),
        }
    }
}
