use super::fuzzer;
use super::RenderMode;
use super::State;

impl State {
    pub fn fuzz(&mut self) {
        self.selected_cmd = 0;
        self.fuzzed_commands = fuzzer::fuzz(&self.commands, &self.search_input)
    }

    pub fn select_down(&mut self) {
        match self.render_mode {
            RenderMode::Normal => {
                if self.fuzzed_commands.is_empty() {
                    self.selected_cmd = (self.selected_cmd + 1) % self.commands.len();
                } else {
                    self.selected_cmd = (self.selected_cmd + 1) % self.fuzzed_commands.len();
                }
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
                if self.fuzzed_commands.is_empty() && self.selected_cmd == 0 {
                    self.selected_cmd = self.commands.len() - 1;
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
