Zellij Command palette

A WIP PoC command palette for Zellij.

This project is mainly me trying out rust for the first time,
so i can guarantee there are issue, problems,
and glaring miss-use of the language in this repo.

The main idea is present a list of Zellij actions that can be performed by the user,
fuzzy find the list and based on the type of action do one of three things:

#### direct commands

Commands that require no additional input, and will simply be performed on key-press

- TogglePaneFrames
- FocusNextPane

#### input commands

commands that require free text input,
opens a secondary menu for text input,
and from there we can perform the command with provided input.

- Rename Session
- Write
- EditFile

#### selection commands

Commands that have a predetermined set of options,
opens secondary selection menu and performs command from there.

- SwitchToMode
- MoveFocus

Right now, the commands are "hard-coded" inside a static list

```rust
pub struct Cmd<Args, Ret> {
  pub name: &'static str,
  pub group: CmdGroup,
  pub options: Option<&'static [&'static str]>,
}
```

The commands are based on what is available in [zellij-tile/src/shim.rs](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/shim.rs).

This is probably not the best approach, and if anyone is able to provide a better
way to handle the list of commands to use, i would greatly appreciate it.

Up until now this has just been me throwing code at the wall and seeing what sticks.
But now that the concept seems to work somewhat,
I would like to take a step back, gather some feedback and find
and actual good approach to handle the commands.

### TODOS

- Build: create a proper wasi build of the project

- Bugs: find and fix bugs, im sure there are loads

- Fuzzing: either build a proper fuzzer or import one

- UI: improve usability and try to provide more information to user

- Commands, implement more or rework how they are handled.

Another addition to the commands could be to define the matching
Zellij function directly in the Cmd-struct itself along the lines of

```rust
pub struct Cmd<Args, Ret> {
  pub name: &'static str,
  pub group: CmdGroup,
  pub options: Option<&'static [&'static str]>,
  pub handler: Box<dyn Fn(Args) -> Ret>, // NEW FIELD
}
```

Where the handler field is simply the corresponding action
that is sent to Zellij.
So adding new commands in the future would only require
adding a new entry with required fields.
