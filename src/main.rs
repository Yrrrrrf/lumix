use clap::{Parser, Subcommand};
use ddc_hi::{Ddc, Display};

// ANSI color codes for pretty printing
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const DIM: &str = "\x1b[2m";

/// A simple, fast tool to control monitor features like brightness via DDC/CI.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

enum ValueChange {
    Delta(i16),
    Absolute(u16)
}

fn parse_change(s: &str) -> Result<ValueChange, Box<dyn std::error::Error>> {
    let trimmed = s.trim();
    
    if trimmed.is_empty() {
        return Err("Empty input".into());
    }
    
    let result = match trimmed {
        // Starts with '+': positive delta
        s if s.starts_with('+') && s.len() > 1 => {
            ValueChange::Delta(s[1..].parse()?)
        }
        
        // Starts with '-': negative delta (parse normally, includes the minus)
        s if s.starts_with('-') && s.len() > 1 => {
            ValueChange::Delta(s.parse()?)
        }
        
        // Ends with '+': positive delta
        s if s.ends_with('+') && s.len() > 1 => {
            ValueChange::Delta(s[..s.len() - 1].parse()?)
        }
        
        // Ends with '-': negative delta
        s if s.ends_with('-') && s.len() > 1 => {
            let value: i16 = s[..s.len() - 1].parse()?;
            ValueChange::Delta(-value)
        }
        
        // Default: absolute value
        s => ValueChange::Absolute(s.parse()?),
    };
    
    Ok(result)
}


#[derive(Subcommand, Debug)]
enum Command {
    /// Get the brightness of one or all monitors
    Get {
        /// The monitor index to target (e.g., 0, 1, 2). If omitted, lists all.
        monitor: Option<usize>,
    },
    /// Set the brightness of one or all monitors
    Set {
        /// The brightness level (0-100)
        brightness: String,

        /// The monitor index to target. If omitted, sets all.
        monitor: Option<usize>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Get { monitor } => handle_get(monitor),
        Command::Set {
            brightness,
            monitor,
        } => handle_set(parse_change(brightness.as_str()).unwrap(), monitor),
    }
}

/// Handles the 'get' subcommand logic.
fn handle_get(monitor_index: Option<usize>) {
    let monitors = find_monitors(monitor_index);

    if monitors.is_empty() {
        println!(
            "{RED}No DDC/CI-enabled monitors found.{RESET}\n{YELLOW}Hint:{RESET} Integrated laptop displays often do not support DDC/CI.",
        );
        return;
    }

    println!("\n{}Monitor Brightness Status:{}\n", BOLD, RESET);
    for (i, mut display) in monitors {
        match display.handle.get_vcp_feature(0x10) {
            Ok(value) => {
                let bar = create_brightness_bar(value.value(), value.maximum());
                println!(
                    "{CYAN}Monitor {i}{RESET}: {BOLD}{:>3}%{RESET} {DIM}[0..{:<3}]{RESET} {}",
                    value.value(),
                    value.maximum(),
                    bar
                );
            }
            Err(e) => {
                // Gracefully handle monitors that are found but don't respond
                println!(
                    "{CYAN}Monitor {i}{RESET}: {RED}Error reading brightness: {}{RESET} {DIM}(likely does not support DDC/CI){RESET}",
                    e
                );
            }
        }
    }
    println!();
}

/// Handles the 'set' subcommand logic.
fn handle_set(brightness: ValueChange, monitor_index: Option<usize>) {
    let monitors = find_monitors(monitor_index);


    if monitors.is_empty() {
        println!("{RED}No DDC/CI-enabled monitors found to set brightness on.{RESET}",);
        return;
    }

    for (i, mut display) in monitors {
        let value:u16 = match brightness {
            ValueChange::Delta(delta) => {
                let current =  {
                    match display.handle.get_vcp_feature(0x10) {
                        Ok(info) => info.value(),
                        Err(e) => {
                            println!(
                                "{RED}✖{RESET} Error getting current brightness for monitor {CYAN}{i}{RESET}: {} (skipping)", e);
                            continue;
                        }
                    }
                };
                (current as i16 + delta as i16).clamp(0, 100) as u16
            }
            ValueChange::Absolute(value) => value,
        };

        match display.handle.set_vcp_feature(0x10, value) {
            Ok(_) => {
                println!(
                    "{GREEN}✔{RESET} Set brightness to {BOLD}{value}%{RESET} for monitor {CYAN}{i}{RESET}"
                );
            }
            Err(e) => {
                println!(
                    "{RED}✖{RESET} Error setting brightness for monitor {CYAN}{i}{RESET}: {}",
                    e
                );
            }
        }
    }
}

/// Finds monitors, either all or a specific one by its index.
/// ddc-hi 0.5+ simplifies this, as enumerate() returns ready-to-use displays.
fn find_monitors(monitor_index: Option<usize>) -> Vec<(usize, Display)> {
    Display::enumerate()
        .into_iter()
        .enumerate()
        .filter(|(i, _)| monitor_index.map_or(true, |index| *i == index))
        .collect()
}

/// Creates a visual progress bar string for brightness.
fn create_brightness_bar(current: u16, max: u16) -> String {
    let bar_width = 20;
    let percentage = if max > 0 { current as f32 / max as f32 } else { 0.0 };
    let filled = (percentage * bar_width as f32).round() as usize;
    let empty = bar_width - filled;
    // The corrected format string with five placeholders for five arguments
    format!(
        "[{}{}{}{}{}]",
        GREEN,
        "█".repeat(filled),
        DIM,
        "░".repeat(empty),
        RESET
    )
}
