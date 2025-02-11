use owo_colors::OwoColorize;

use zellij_tile::prelude::*;
pub fn input_render(selected_cmd: usize, cmd_input: String, fuzzed_commands: Vec<(String, f32)>) {
    println!(
        "{} {}",
        ">".cyan().bold(),
        if cmd_input.is_empty() {
            format!(
                "{} {}",
                fuzzed_commands[selected_cmd].0,
                ": _".cyan().bold()
            )
        } else {
            format!("{}{}{}", fuzzed_commands[selected_cmd].0, ": ", cmd_input)
                .cyan()
                .bold()
                .to_string()
        }
    );
}
pub fn message_render(message_content: String) {
    println!(
        "{} {}",
        ">".cyan().bold(),
        message_content.to_string().cyan().bold()
    );
    print_text_with_coordinates(Text::new("<ENTER>/<ESC>"), 0, 1, None, None)
}

pub fn select_render(
    option_selection: Vec<String>,
    selected_option: usize,
    row: usize,
    _cols: usize,
) {
    let (first, last) = if option_selection.len() >= row {
        let count = row.saturating_sub(3);
        let first = selected_option.saturating_sub(count / 2);
        let last = first + count;
        (first, last)
    } else {
        let first = 0;
        let last = option_selection.len();
        (first, last)
    };

    let mut table = Table::new().add_row(vec![" ", " "]);
    for i in first..last {
        if i >= option_selection.len() {
            continue;
        }

        if i == selected_option {
            table = table.add_styled_row(vec![
                Text::new(format!("{}", "->".red())),
                Text::new(&option_selection[i]),
            ]);
        } else {
            table = table.add_styled_row(vec![
                Text::new(format!("{}", "->".green())),
                Text::new(&option_selection[i]),
            ]);
        }
    }

    print_table(table);

    print_text_with_coordinates(Text::new("<↓↑>/<ENTER>/<ESC>"), 0, row, None, None)
}

pub fn normal_render(
    search_input: &String,
    selected_cmd: usize,
    mut fuzzed_commands: Vec<(String, f32)>,
    commands: &Vec<String>,
    row: usize,
    _cols: usize,
) {
    println!(
        "{} {}",
        ">".cyan().bold(),
        if search_input.is_empty() {
            format!("{} {}", selected_cmd, "Search: _".cyan().bold(),)
        } else {
            format!("{}{}", "Search: ", search_input)
                .cyan()
                .bold()
                .to_string()
        }
    );

    let mut table = Table::new().add_row(vec![" ", " "]);

    if search_input.is_empty() {
        fuzzed_commands.clear();
        for i in commands {
            fuzzed_commands.push((i.clone(), 0.0));
        }
    }

    let (first, last) = if fuzzed_commands.len() >= row - 4 {
        let count = row.saturating_sub(4);
        let first = selected_cmd.saturating_sub(count / 2);
        let last = first + count;
        (first, last)
    } else {
        let first = 0;
        let last = fuzzed_commands.len();
        (first, last)
    };

    for i in first..last {
        if i >= fuzzed_commands.len() {
            continue;
        }

        if i == selected_cmd {
            table = table.add_styled_row(vec![
                Text::new(format!("{}", "->".red())),
                Text::new(&fuzzed_commands[i].0),
            ]);
        } else {
            table = table.add_styled_row(vec![
                Text::new(format!("{}", "->".green())),
                Text::new(&fuzzed_commands[i].0),
            ]);
        }
    }
    table = table.add_row(vec![" ", " "]);

    print_table(table);
    print_text_with_coordinates(Text::new("<↓↑>/<ENTER>/<ESC>"), 0, row, None, None)
}
