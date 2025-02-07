#[derive(Clone, Debug)]
pub struct Cmd {
    pub name: &'static str,
    pub group: CmdGroup,
    pub options: Option<&'static [&'static str]>,
}

#[derive(Clone, Debug)]
pub enum CmdGroup {
    Direct,
    Input,
    Select,
}

pub fn get_list() -> Vec<String> {
    return CMD_LIST.iter().map(|cmd| cmd.name.to_string()).collect();
}

pub fn get_by_name(input: &str) -> Option<&Cmd> {
    CMD_LIST.iter().find(|&x| x.name == input)
}
pub fn get_options_by_name(input: &str) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();

    if let Some(find) = CMD_LIST.iter().find(|&x| x.name == input) {
        if let Some(opts) = find.options {
            opts.iter().for_each(|&o| vector.push(o.to_string()))
        }
    }
    vector
}

pub const CMD_LIST: &[Cmd] = &[
    Cmd {
        name: "Quit",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "Write",
        group: CmdGroup::Input,
        options: None,
    },
    Cmd {
        name: "TogglePaneFrames",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "BreakPaneLeft",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "ScrollUp",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "ScrollUpAt",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "PageScrollUp",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "ScrollDown",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "ScrollDownAt",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "PageScrollDown",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "SwitchToMode",
        group: CmdGroup::Select,
        options: Some(&[
            "Normal",
            "Locked",
            "Resize",
            "Pane",
            "Tab",
            "Search",
            "Scroll",
            "RenameTab",
            "Session",
            "Move",
            "Prompt",
            "Tmux",
            "EnterSearch",
        ]),
    },
    Cmd {
        name: "SwitchFocus",
        group: CmdGroup::Select,
        options: None,
    },
    Cmd {
        name: "FocusNextPane",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "FocusPreviousPane",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "RenameSession",
        group: CmdGroup::Input,
        options: None,
    },
    Cmd {
        name: "Resize",
        group: CmdGroup::Direct,
        options: None,
    },
    Cmd {
        name: "MoveFocus",
        group: CmdGroup::Select,
        options: Some(&["Up", "Down", "Left", "Right"]),
    },
    Cmd {
        name: "NewPane",
        group: CmdGroup::Direct,
        options: None,
    },
];

//     "WriteChars".
//     "SwitchModeForAllClients".
//     "MoveFocusOrTab".
//     "MovePane".
//     "MovePaneBackwards".
//     "ClearScreen".
//     "DumpScreen".
//     "DumpLayout".
//     "EditScrollback".
//
//     "ScrollToBottom".
//     "ScrollToTop".
//     "HalfPageScrollUp".
//     "HalfPageScrollDown".
//
//     "ToggleFocusFullscreen".
//     "ToggleActiveSyncTab".
//
//     "EditFile".
//
//     "NewPane".
//     "NewFloatingPane".
//     "NewTiledPane".
//     "NewInPlacePane".
//
//     "TogglePaneEmbedOrFloating".
//     "ToggleFloatingPanes".
//
//     "CloseFocus".
//     "PaneNameInput".
//     "UndoRenamePane".
//     "NewTab".
//     "GoToNextTab".
//     "GoToPreviousTab".
//     "CloseTab".
//     "GoToTab".
//     "GoToTabName".
//     "ToggleTab".
//     "TabNameInput".
//     "UndoRenameTab".
//     "MoveTab".
//     "Run".
//     "Detach".
//     "LeftClick".
//     "RightClick".
//     "MiddleClick".
//     "LaunchOrFocusPlugin".
//     "LaunchPlugin".
//     "LeftMouseRelease".
//     "RightMouseRelease".
//     "MiddleMouseRelease".
//     "MouseHoldLeft".
//     "MouseHoldRight".
//     "MouseHoldMiddle".
//     "Copy".
//     "Confirm".
//     "Deny".
//     "SkipConfirm".
//     "SearchInput".
//     "Search".
//     "SearchToggleOption".
//     "ToggleMouseMode".
//     "PreviousSwapLayout".
//     "NextSwapLayout".
//     "QueryTabNames".
//     "NewTiledPluginPane".
//     "NewFloatingPluginPane".
//     "NewInPlacePluginPane".
//     "StartOrReloadPlugin".
//     "CloseTerminalPane".
//     "ClosePluginPane".
//     "FocusTerminalPaneWithId".
//     "FocusPluginPaneWithId".
//     "RenameTerminalPane".
//     "RenamePluginPane".
//     "RenameTab".
//     "BreakPane".
//     "BreakPaneRight".
//     "BreakPaneLeft".
//     "RenameSession".
//     "CliPipe".
//     "KeybindPipe".
//     "ListClients".
//
