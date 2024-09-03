use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    process::{Command, Stdio},
};

use anyhow::{Context, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use gumdrop::Options;
use serde::Deserialize;
use serde_yaml::Value;

#[derive(Deserialize)]
struct Snippet {
    name: String,
    content: String,
}

#[derive(Deserialize)]
struct SnippetsConfig {
    snippets: Vec<Snippet>,
}

#[derive(Debug, Options)]
struct Args {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "print version")]
    version: bool,
    #[options(help = "config file location ")]
    configfile: Option<String>,
    #[options(help = "print command to stdout, do not run it")]
    print_only: bool,
}

fn read_snippets_config(filename: &str) -> Result<Vec<Snippet>> {
    let file = File::open(filename).context(format!("cannot open config file {}", filename))?;
    let config: SnippetsConfig =
        serde_yaml::from_reader(file).context(format!("cannot parse config file {}", filename))?;
    Ok(config.snippets)
}

fn run_fuzzel_with_input(input: String) -> String {
    let mut child = Command::new("fuzzel")
        .args(["-d", "--no-fuzzy"])
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("cannot launch fuzzel command");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes()).unwrap();
    }
    let output = child.wait_with_output().expect("failed to read output");
    String::from_utf8(output.stdout).unwrap()
}

fn main() -> Result<()> {
    let args = Args::parse_args_default_or_exit();

    let home = std::env::var("HOME").unwrap();
    let configfile = args
        .configfile
        .unwrap_or_else(|| format!("{}/.snippets", home));

    let snippets = read_snippets_config(&configfile)?;
    let snippet_names: Vec<String> = snippets.iter().map(|s| s.name.clone()).collect();
    let input = snippet_names.join("\n");

    let selected_snippet_name = run_fuzzel_with_input(input).trim().to_string();

    if let Some(snippet) = snippets.iter().find(|s| s.name == selected_snippet_name) {
        
        // ...
        
        let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to initialize clipboard");
        ctx.set_contents(snippet.content.clone()).expect("Failed to copy to clipboard");
        let mut child = Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()
            .expect("cannot launch xclip command");

        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(snippet.content.as_bytes()).unwrap();
        }
        child.wait().expect("failed to copy to clipboard");
    }

    Ok(())
}