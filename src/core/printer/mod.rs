use console::style;

pub fn print_blocked_text(text: &str, message: &str) {
    println!("\n\x1b[30;42m {} \x1b[0m {}\n", text, style(message));
}

pub fn print_action(action: &str, message: &str) {
    println!(
        "{} {}",
        style(action).green().bright(),
        style(message).dim(),
    );
}

pub fn print_success_text(message: &str) {
    println!(
        "\n{} {}\n",
        style("✔").green().bright(),
        style(message).green()
    );
}

pub fn print_error_text(message: &str) {
    println!("\n{} {}\n", style("✖").red().bright(), style(message).red());
}
