use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
};
use std::io::{self, Result};

pub struct JellyOutput;

impl JellyOutput {
    pub fn success(message: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Green),
            Print("✓ "),
            ResetColor,
            Print(message),
            Print("\n")
        )
    }

    pub fn error(message: &str) -> Result<()> {
        execute!(
            io::stderr(),
            SetForegroundColor(Color::Red),
            Print("✗ "),
            ResetColor,
            Print(message),
            Print("\n")
        )
    }

    pub fn info(message: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Blue),
            Print("ℹ "),
            ResetColor,
            Print(message),
            Print("\n")
        )
    }

    pub fn warning(message: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print("⚠ "),
            ResetColor,
            Print(message),
            Print("\n")
        )
    }

    pub fn header(message: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Magenta),
            Print("🍇 "),
            ResetColor,
            Print(message.bold()),
            Print("\n")
        )
    }

    pub fn step(message: &str) -> Result<()> {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Cyan),
            Print("→ "),
            ResetColor,
            Print(message),
            Print("\n")
        )
    }

    pub fn plain(message: &str) -> Result<()> {
        execute!(
            io::stdout(),
            Print(message),
            Print("\n")
        )
    }

    pub fn newline() -> Result<()> {
        execute!(io::stdout(), Print("\n"))
    }
}