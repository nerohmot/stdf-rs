#!/usr/bin/env run-cargo-script
// cargo-deps: toml = "0.8.19"
extern crate toml;

use std::fs;
use std::io::{self, Write};
use toml::Value;

fn main() -> io::Result<()> {
    // Read the Cargo.toml file
    let cargo_toml_content = fs::read_to_string("Cargo.toml")?;
    let cargo_toml: Value = toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

    // Extract the authors entry
    let authors = cargo_toml
        .get("package")
        .and_then(|pkg| pkg.get("authors"))
        .and_then(|authors| authors.as_array())
        .expect("Failed to find authors in Cargo.toml");

    // Prepare the CODEOWNERS content
    let mut codeowners_content = String::from("* ");

    for author in authors {
        if let Some(author_str) = author.as_str() {
            let email_start = author_str.find('<').unwrap_or(0);
            //let email_end = author_str.find('>').unwrap_or(author_str.len());
            //let email = &author_str[email_start..=email_end];
            let username = &author_str[0..email_start];
            codeowners_content.push_str(username);
            codeowners_content.push(' ');
        }
    }

    // Write the CODEOWNERS file
    let mut codeowners_file = fs::File::create(".github/CODEOWNERS")?;
    codeowners_file.write_all(codeowners_content.trim_end().as_bytes())?;

    Ok(())
}