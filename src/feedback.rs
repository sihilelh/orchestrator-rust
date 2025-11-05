use colored::*;

/// Prints a success message with a green checkmark
pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg.green());
}

/// Prints an info message with a blue arrow
pub fn info(msg: &str) {
    println!("{} {}", "→".blue().bold(), msg.bright_blue());
}

/// Prints a processing message with a spinner-like indicator
pub fn processing(msg: &str) {
    println!("{} {}", "⚙".yellow().bold(), msg.bright_blue());
}
