use purr::read_stdin;
use std::io::BufRead;

/// Basic `read_stdin` test.
#[ignore = "Need to manually enter input for testing"]
#[test]
fn read_stdin_basic() -> anyhow::Result<()> {
    let mut buf = String::new();
    read_stdin()?.read_line(&mut buf)?;
    assert!(buf.as_str() == "A test\n", "buf = \"{}\"", buf.as_str());

    Ok(())
}
