use std::{
    fs::{self, File},
    io::Write,
    os::unix::prelude::PermissionsExt,
};

use assert_cmd::{prelude::*, Command}; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*;

/// Test with only standard input.
#[test]
fn standard_input_only() -> anyhow::Result<()> {
    // init the bin cli
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.write_stdin("A test");
    cmd.assert().stdout(predicate::str::contains("test"));

    Ok(())
}

/// Single file input test.
#[test]
fn single_file_read() -> anyhow::Result<()> {
    // create temp test file(s)
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test")?;

    // init the bin cli
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // test
    cmd.arg(file.path());
    cmd.assert().stdout(predicate::str::contains("test"));

    Ok(())
}

/// Single binary file input test.
#[test]
fn single_binary_file_read() -> anyhow::Result<()> {
    // bin data to write to file and test
    let bin_data = &[0u8, 1, 2, 128, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] as &[u8];
    // create temp test file(s)
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_binary(bin_data)?;

    // init the bin cli
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    // test
    cmd.arg(file.path());
    cmd.assert().success();

    Ok(())
}

/// Multiple file input test.
#[test]
fn multi_file_read() -> anyhow::Result<()> {
    let file_text_template = |i| format!("Sample {} test", i);
    // create temp test file(s)
    let mut files = vec![];
    for i in 0..5 {
        let file = assert_fs::NamedTempFile::new(&format!("Sample {}.txt", i))?;
        file.write_str(&file_text_template(i))?;
        files.push(file);
    }

    // init the bin cli
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.args(files.iter().map(|f| f.path()));
    for i in 0..files.len() {
        cmd.assert()
            .stdout(predicate::str::contains(file_text_template(i)));
    }

    Ok(())
}

/// Test read 1 file + 1 stdin via `-`
#[test]
fn single_file_single_stdin() -> anyhow::Result<()> {
    // create temp test file(s)
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("Sample file text")?;

    // init the bin cli
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg(file.path());
    cmd.arg("-");
    cmd.write_stdin("Standard input text");
    cmd.assert()
        .stdout(predicate::str::contains("Sample file text"));
    cmd.assert()
        .stdout(predicate::str::contains("Standard input text"));

    Ok(())
}

/// Multiple file+stdin test.
#[test]
fn mixed_multi_file_read() -> anyhow::Result<()> {
    let file_text_template = |i| format!("File {} test", i);
    // create temp test file(s)
    let mut files = vec![];
    for i in 0..5 {
        let file = assert_fs::NamedTempFile::new(&format!("File {}.txt", i))?;
        file.write_str(&file_text_template(i))?;
        files.push(file);
    }

    let stdin_text_template = |i| format!("Stdin {} test", i);
    let mut stdin_texts = vec![];
    for i in 0..5 {
        stdin_texts.push(stdin_text_template(i));
    }

    // init the bin cli
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    let total_args = files.len() + stdin_texts.len();
    for i in 0..total_args {
        if i % 2 == 0 {
            if let Some(file) = &files.get(i) {
                // write file paths for even
                cmd.arg(file.path());
            }
        } else if stdin_texts.get(i).is_some() {
            // write hyphen(stdin) for odd
            cmd.arg("-");
        }
    }
    for i in 0..total_args {
        if i % 2 == 0 {
            if files.get(i).is_some() {
                cmd.assert()
                    .stdout(predicate::str::contains(file_text_template(i)));
            }
        } else if let Some(text) = stdin_texts.get(i) {
            // write text to stdin
            cmd.write_stdin(text.clone());
            // assert
            cmd.assert().stdout(predicate::str::contains(text));
        }
    }

    Ok(())
}

/// Test error on trying to read non-permitted file.
#[test]
fn not_permitted_file_read() -> anyhow::Result<()> {
    // create temp test file(s)
    let mut file = File::create("target/sample.txt")?;
    writeln!(file, "Sample file text")?;

    let meta = file.metadata()?;
    let mut perms = meta.permissions();
    perms.set_mode(0o333);
    file.set_permissions(perms)?;
    file.flush()?;

    // init the bin cli
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("target/sample.txt");
    cmd.assert()
        .stderr(predicate::str::contains("Error: Permission denied"));

    fs::remove_file("target/sample.txt")?;

    Ok(())
}

/// Multiple file input test with line numbers.
#[test]
fn multi_file_read_show_lines() -> anyhow::Result<()> {
    let file_text_template = |i| format!("Sample {} test", i);
    // create temp test file(s)
    let mut files = vec![];
    for i in 0..5 {
        let file = assert_fs::NamedTempFile::new(&format!("Sample {}.txt", i))?;
        file.write_str(&file_text_template(i))?;
        files.push(file);
    }

    // init the bin cli
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("-n").args(files.iter().map(|f| f.path()));
    for i in 0..files.len() {
        cmd.assert().stdout(predicate::str::contains(i.to_string()));
        cmd.assert()
            .stdout(predicate::str::contains(file_text_template(i)));
    }

    Ok(())
}
