use super::fuzzer;
use super::RenderMode;
use super::State;

use owo_colors::OwoColorize;
use zellij_tile::shim::*;

impl State {
    pub fn fuzz(&mut self) {
        self.selected_cmd = 0;
        self.fuzzed_commands = fuzzer::fuzz(&self.commands, &self.search_input)
    }

    pub fn select_down(&mut self) {
        match self.render_mode {
            RenderMode::Normal => {
                if self.fuzzed_commands.is_empty() {
                    return;
                }
                self.selected_cmd = (self.selected_cmd + 1) % self.fuzzed_commands.len();
            }
            RenderMode::OptionSelect => {
                if self.option_selection.is_empty() {
                    return;
                }
                self.selected_option = (self.selected_option + 1) % self.option_selection.len();
            }
            _ => {}
        }
    }

    pub fn select_up(&mut self) {
        match self.render_mode {
            RenderMode::Normal => {
                if self.fuzzed_commands.is_empty() {
                    return;
                }
                if self.selected_cmd == 0 {
                    self.selected_cmd = self.fuzzed_commands.len() - 1;
                    return;
                }
                self.selected_cmd -= 1;
            }
            RenderMode::OptionSelect => {
                if self.option_selection.is_empty() {
                    return;
                }
                if self.selected_option == 0 {
                    self.selected_option = self.option_selection.len() - 1;
                    return;
                }
                self.selected_option -= 1
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        self.render_mode = RenderMode::Normal;
        self.cmd_input.clear();
        self.search_input.clear();
        self.option_selection.clear();
    }

    pub fn error_msg(&mut self, msg: String) {
        self.render_mode = RenderMode::Message;
        self.message_content = msg;
    }

    pub fn message_render(&mut self) {
        println!(
            "{} {}",
            ">".cyan().bold(),
            self.message_content.to_string().cyan().bold()
        );
    }

    pub fn input_render(&mut self) {
        println!(
            "{} {}",
            ">".cyan().bold(),
            if self.cmd_input.is_empty() {
                format!(
                    "{} {}",
                    self.fuzzed_commands[self.selected_cmd].0,
                    ": _".cyan().bold()
                )
            } else {
                format!(
                    "{}{}{}",
                    self.fuzzed_commands[self.selected_cmd].0, ": ", self.cmd_input
                )
                .cyan()
                .bold()
                .to_string()
            }
        );
    }

    pub fn select_render(&mut self, row: usize, _cols: usize) {
        let (first, last) = if self.option_selection.len() >= row {
            let count = row.saturating_sub(3);
            let first = self.selected_option.saturating_sub(count / 2);
            let last = first + count;
            (first, last)
        } else {
            let first = 0;
            let last = self.option_selection.len();
            (first, last)
        };

        let mut table = Table::new().add_row(vec![" ", " "]);
        for i in first..last {
            if i >= self.option_selection.len() {
                continue;
            }

            if i == self.selected_option {
                table = table.add_styled_row(vec![
                    Text::new(format!("{}", "->".red())),
                    Text::new(&self.option_selection[i]),
                ]);
            } else {
                table = table.add_styled_row(vec![
                    Text::new(format!("{}", "->".green())),
                    Text::new(&self.option_selection[i]),
                ]);
            }
        }

        print_table(table);
    }

    pub fn normal_render(&mut self, row: usize, _cols: usize) {
        println!(
            "{} {}",
            ">".cyan().bold(),
            if self.search_input.is_empty() {
                format!("{} {}", self.selected_cmd, "Search: _".cyan().bold(),)
            } else {
                format!("{}{}", "Search: ", self.search_input)
                    .cyan()
                    .bold()
                    .to_string()
            }
        );

        let mut table = Table::new().add_row(vec![" ", " "]);

        if self.search_input.is_empty() {
            self.fuzzed_commands.clear();
            for i in &self.commands {
                self.fuzzed_commands.push((i.clone(), 0.0));
            }
        }

        let (first, last) = if self.fuzzed_commands.len() >= row {
            let count = row.saturating_sub(3);
            let first = self.selected_cmd.saturating_sub(count / 2);
            let last = first + count;
            (first, last)
        } else {
            let first = 0;
            let last = self.fuzzed_commands.len();
            (first, last)
        };

        for i in first..last {
            if i >= self.fuzzed_commands.len() {
                continue;
            }

            if i == self.selected_cmd {
                table = table.add_styled_row(vec![
                    Text::new(format!("{}", "->".red())),
                    Text::new(&self.fuzzed_commands[i].0),
                ]);
            } else {
                table = table.add_styled_row(vec![
                    Text::new(format!("{}", "->".green())),
                    Text::new(&self.fuzzed_commands[i].0),
                ]);
            }
        }

        print_table(table);
    }

    pub fn update_tile(&mut self) {
        let pane_manifest = self.tile.pane_manifest.clone();
        let tab_info = self.tile.tab_info.clone();

        for tab in tab_info {
            if tab.active {
                self.tile.focused_tab = tab.clone();
            }
        }

        let panes = pane_manifest.panes.get(&self.tile.focused_tab.position);
        if let Some(panes) = panes {
            for pane in panes {
                if pane.is_focused & !pane.is_plugin {
                    self.tile.focused_pane = pane.clone();
                }
            }
        }
    }
}
