#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

#[cfg(target_os = "none")]
use user::*;
#[cfg(target_os = "none")]
use libposix::{File, Read, Write, stdin, stdout, args, exit};
#[cfg(target_os = "none")]
use alloc::string::String;

#[cfg(not(target_os = "none"))]
use std::io::{self, Read, Write};
#[cfg(not(target_os = "none"))]
use std::env;

#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
fn _start(_args: Args) {
    let argv = args();
    run(&argv);
    exit(0);
}

#[cfg(not(target_os = "none"))]
fn main() {
    let argv: Vec<String> = env::args().collect();
    run(&argv);
}

fn run(argv: &[String]) {
    let mut lines: usize = 10;
    let mut i = 1;

    while i < argv.len() && argv[i].starts_with('-') && argv[i] != "--" {
        if argv[i] == "--" { i += 1; break; }
        for c in argv[i][1..].chars() {
            match c {
                'n' => {
                    i += 1;
                    if i < argv.len() {
                        lines = argv[i].parse().unwrap_or(10);
                    }
                }
                'c' => {
                    i += 1;
                    if i < argv.len() {
                        let nbytes: usize = argv[i].parse().unwrap_or(0);
                        #[cfg(target_os = "none")]
                        {
                            let mut sin = stdin();
                            let mut buf = alloc::vec![0u8; nbytes];
                            let _ = sin.read(&mut buf);
                            let mut out = stdout();
                            let _ = out.write(&buf);
                            return;
                        }
                        #[cfg(not(target_os = "none"))]
                        {
                            let mut sin = io::stdin();
                            let mut buf = vec![0u8; nbytes];
                            let _ = sin.read(&mut buf);
                            let mut out = io::stdout();
                            let _ = out.write(&buf);
                            return;
                        }
                    }
                }
                'q' => {}
                'v' => {}
                _ => {
                    eprintln!("head: invalid option -- '{}'", c);
                    #[cfg(target_os = "none")]
                    exit(1);
                    #[cfg(not(target_os = "none"))]
                    std::process::exit(1);
                }
            }
        }
        i += 1;
    }

    let files: Vec<String> = argv[i..].to_vec();

    if files.is_empty() {
        #[cfg(target_os = "none")]
        head_stdin(lines);
        #[cfg(not(target_os = "none"))]
        head_stdin(lines);
        return;
    }

    for fname in &files {
        #[cfg(target_os = "none")]
        head_file(fname, lines);
        #[cfg(not(target_os = "none"))]
        head_file(fname, lines);
    }
}

#[cfg(not(target_os = "none"))]
fn head_file(path: &str, lines: usize) {
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("head: {}: error opening file", path);
            std::process::exit(1);
        }
    };

    let mut out = io::stdout();
    let mut buf = [0u8; 8192];
    let mut line_count = 0;
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == b'\n' {
                        line_count += 1;
                        if line_count >= lines {
                            return;
                        }
                    }
                    let _ = out.write(&[b]);
                }
            }
            Err(_) => break,
        }
    }
}

#[cfg(not(target_os = "none"))]
fn head_stdin(lines: usize) {
    let mut sin = io::stdin();
    let mut out = io::stdout();
    let mut buf = [0u8; 8192];
    let mut line_count = 0;
    loop {
        match sin.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == b'\n' {
                        line_count += 1;
                        if line_count >= lines {
                            return;
                        }
                    }
                    let _ = out.write(&[b]);
                }
            }
            Err(_) => break,
        }
    }
}

#[cfg(target_os = "none")]
fn head_file(path: &str, lines: usize) {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("head: {}: error opening file", path);
            exit(1);
        }
    };

    let mut out = stdout();
    let mut buf = [0u8; 8192];
    let mut line_count = 0;
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == b'\n' {
                        line_count += 1;
                        if line_count >= lines {
                            return;
                        }
                    }
                    let _ = out.write(&[b]);
                }
            }
            Err(_) => break,
        }
    }
}

#[cfg(target_os = "none")]
fn head_stdin(lines: usize) {
    let mut sin = stdin();
    let mut out = stdout();
    let mut buf = [0u8; 8192];
    let mut line_count = 0;
    loop {
        match sin.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == b'\n' {
                        line_count += 1;
                        if line_count >= lines {
                            return;
                        }
                    }
                    let _ = out.write(&[b]);
                }
            }
            Err(_) => break,
        }
    }
}