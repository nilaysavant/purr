use std::io::{self, Write};

/// Print manager.
///
/// Handles printing the files/input to stdout.
#[derive(Debug)]
pub struct PrintManager {
    // Flag to show/hide line numbers.
    pub display_line_numbers: bool,
    /// Current line number, starting from 1.
    line_num: u128,
}

impl PrintManager {
    /// Create new PrintManager instance.
    pub fn new() -> Self {
        Self {
            display_line_numbers: false,
            line_num: 0,
        }
    }

    /// Write single line to std out.
    pub fn write_line(&mut self, buf: String) -> anyhow::Result<()> {
        self.line_num += 1; // incr line counter (start from 1)
        let formatted = if self.display_line_numbers {
            format!("{}  {}", self.line_num, buf)
        } else {
            buf
        };
        writeln!(io::stdout(), "{}", formatted)?;

        Ok(())
    }
}

impl Default for PrintManager {
    fn default() -> Self {
        Self::new()
    }
}
