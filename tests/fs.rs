use std::io::Read;

use assert_fs::prelude::*;

use purr::{errors::TestError, read_file, validate_file_path};

/// Basic read file test.
#[test]
fn read_file_basic() -> anyhow::Result<()> {
    // create temp test files
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test")?;

    let mut buf = String::new();
    read_file(file.path())?.read_to_string(&mut buf)?;

    assert!(buf.as_str() == "A test");

    Ok(())
}

/// Test `validate_file_path` when file exists.
#[test]
fn validate_file_path_when_file_exists() -> anyhow::Result<()> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test")?; // required (else the file is not created and test fails)

    let path_str = file
        .path()
        .to_str()
        .ok_or_else(|| TestError::Custom("file path cannot be stringified!".into()))?;
    let path_buf = validate_file_path(path_str.into());

    assert!(path_buf.is_ok());

    Ok(())
}

/// Test `validate_file_path` when file does not exist.
#[test]
fn validate_file_path_when_file_dne() -> anyhow::Result<()> {
    let path_buf = validate_file_path("-".into());

    assert!(path_buf.is_err());

    Ok(())
}
