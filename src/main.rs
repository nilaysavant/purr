use std::io::BufRead;

use anyhow::anyhow;
use clap::Parser;
use purr::{read_file, read_stdin, validate_file_path, PrintManager, PurrArgs};

fn main() -> anyhow::Result<()> {
    let args = PurrArgs::parse();
    let mut print_manager = PrintManager::new();
    print_manager.display_line_numbers = args.number_output_lines;
    if let Some(file_paths) = args.files {
        for path_str in file_paths.iter() {
            if path_str == "-" {
                // if path is '-' read stdin
                handle_stdin(&mut print_manager)?;
            } else if let Ok(path_buf) = validate_file_path(path_str.into()) {
                // if path is valid file, read file
                let buf_reader = read_file(path_buf.as_path())?;
                let lines = buf_reader.lines().map(|a| a.unwrap()).collect::<Vec<_>>();
                for line in lines {
                    print_manager.write_line(line)?;
                }
            } else {
                return Err(anyhow!("No such file or directory: {}", path_str));
            }
        }
    } else {
        // if no file arg(s) provided.
        handle_stdin(&mut print_manager)?;
    }
    Ok(())
}

/// Common handler for concatenation via standard input.
fn handle_stdin(print_manager: &mut PrintManager) -> anyhow::Result<()> {
    // no file path provided, read stdin
    loop {
        // loop through each line reading input lines
        let mut buf = String::new();
        let read_count = read_stdin()?.read_line(&mut buf)?;
        if read_count == 0 {
            // means EOF has reached, break out.
            break;
        }
        // trim end needed to remove extra newline, as writeln adds \n
        print_manager.write_line(buf.trim_end().into())?;
    }
    Ok(())
}
