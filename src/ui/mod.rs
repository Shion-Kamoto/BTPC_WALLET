use colored::{Color, Colorize};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};
use qrcode::{QrCode, Color as QrColor};
use std::io::{self, Write};
use std::process;

/// Display a header with the application name
pub fn show_header() {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║                  BTPC WALLET CLI                    ║".bright_blue().bold());
    println!("{}", "║       Quantum-Resistant Blockchain Wallet           ║".bright_blue());
    println!("{}", "╚══════════════════════════════════════════════════════╝".bright_blue());
    println!();
}

/// Display a success message
pub fn show_success(message: &str) {
    println!("{} {}", "✓".green(), message.green());
}

/// Display an error message
pub fn show_error(message: &str) {
    println!("{} {}", "✗".red(), message.red());
}

/// Display an info message
pub fn show_info(message: &str) {
    println!("{} {}", "ℹ".cyan(), message.cyan());
}

/// Display a warning message
pub fn show_warning(message: &str) {
    println!("{} {}", "⚠".yellow(), message.yellow());
}

/// Display a section header
pub fn show_section_header(title: &str) {
    println!();
    println!("{}", format!("── {} ──", title).bright_cyan().bold());
    println!();
}

/// Display a table row
pub fn show_table_row(key: &str, value: &str) {
    println!("{}: {}", key.bright_white().bold(), value);
}

/// Display a table row with colored value
pub fn show_table_row_colored(key: &str, value: &str, color: Color) {
    println!("{}: {}", key.bright_white().bold(), value.color(color));
}

/// Prompt for confirmation
pub fn confirm(prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap_or(false)
}

/// Prompt for text input
pub fn input(prompt: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

/// Prompt for password input
pub fn password(prompt: &str) -> String {
    Password::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .unwrap()
}

/// Display a menu and get user selection
pub fn menu(title: &str, items: &[&str]) -> usize {
    show_section_header(title);
    Select::with_theme(&ColorfulTheme::default())
        .items(items)
        .default(0)
        .interact()
        .unwrap()
}

/// Display a progress spinner (lightweight simulation)
pub fn show_spinner(message: &str) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    for frame in frames.iter().cycle().take(10) {
        print!("\r{} {} ", frame.yellow(), message);
        io::stdout().flush().ok();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!();
}

/// Display a loading animation
pub fn show_loading(message: &str) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    for frame in frames.iter().cycle().take(20) {
        print!("\r{} {} ", frame.yellow(), message);
        io::stdout().flush().ok();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    println!();
}

/// Display a transaction confirmation
pub fn show_transaction_confirmation(amount: &str, recipient: &str, fee: &str) {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════╗".bright_green());
    println!("{}", "║               TRANSACTION CONFIRMATION              ║".bright_green().bold());
    println!("{}", "╠══════════════════════════════════════════════════════╣".bright_green());
    show_table_row_colored("Amount", amount, Color::Green);
    show_table_row("Recipient", recipient);
    show_table_row("Fee", fee);
    println!("{}", "╚══════════════════════════════════════════════════════╝".bright_green());
    println!();
}

/// Display wallet balance
pub fn show_balance(balance: &str, pending: &str) {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════╗".bright_blue());
    println!("{}", "║                    WALLET BALANCE                   ║".bright_blue().bold());
    println!("{}", "╠══════════════════════════════════════════════════════╣".bright_blue());
    show_table_row_colored("Available", balance, Color::Green);
    show_table_row_colored("Pending", pending, Color::Yellow);
    println!("{}", "╚══════════════════════════════════════════════════════╝".bright_blue());
    println!();
}

/// Display a transaction history table
pub fn show_transaction_history(transactions: Vec<(&str, &str, &str, &str)>) {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                            TRANSACTION HISTORY                              ║".bright_cyan().bold());
    println!("{}", "╠══════════════════════════════════════════════════════════════════════════════╣".bright_cyan());

    for (date, amount, recipient, status) in transactions {
        let status_color = match status {
            "Confirmed" => Color::Green,
            "Pending" => Color::Yellow,
            "Failed" => Color::Red,
            _ => Color::White,
        };

        println!(
            "{} {:<10} {:<15} {:<30} {}",
            "▸".bright_white(),
            date,
            amount.green(),
            recipient,
            status.color(status_color)
        );
    }

    println!("{}", "╚══════════════════════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
}

/// Display a QR code using Unicode blocks in the terminal
pub fn show_qr_code(data: &str) {
    const INNER_WIDTH: usize = 44;

    println!();
    println!("{}", "╔══════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║                     QR CODE                          ║".bright_magenta().bold());
    println!("{}", "╠══════════════════════════════════════════════════════╣".bright_magenta());

    match QrCode::new(data) {
        Ok(code) => {
            let width = code.width();
            for y in 0..width {
                let mut line = String::new();
                for x in 0..width {
                    let pixel = match code[(x, y)] {
                        QrColor::Dark => '█',
                        QrColor::Light => ' ',
                    };
                    line.push(pixel);
                }
                println!("{}", format!("║ {:<width$} ║", line, width = INNER_WIDTH).bright_magenta());
            }
        }
        Err(_) => {
            println!(
                "{}",
                format!("║ {:<width$} ║", "Could not generate QR code", width = INNER_WIDTH).bright_red()
            );
        }
    }

    println!("{}", "║                                                      ║".bright_magenta());
    println!("{}", "║        Scan this QR code with a wallet app          ║".bright_magenta());
    println!("{}", "╚══════════════════════════════════════════════════════╝".bright_magenta());

    println!();
    println!("{}", "Full address for manual entry:".bright_cyan());
    println!("{}", data.cyan());
    println!();
}

/// Exit the application with a message
pub fn exit_with_message(message: &str, is_error: bool) -> ! {
    if is_error {
        show_error(message);
    } else {
        show_success(message);
    }
    println!();
    process::exit(if is_error { 1 } else { 0 });
}
