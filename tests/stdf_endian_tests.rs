use std::process::Command;
use std::path::PathBuf;

#[test]
fn test_stdf_endian() {
    // Construct the path to the sample data file
    let mut sample_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    sample_file_path.push("tests/data/test.stdf");

    // Run the CLI binary with the sample input file
    let output = Command::new("cargo")
        .args(&["run", "--bin", "stdf_endian", "--", sample_file_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    // Check that the command was successful
    assert!(output.status.success());

    // Convert the output to a string
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check the output
    assert!(stdout.contains("LE"));
}