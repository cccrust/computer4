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
    if argv.len() < 2 {
        #[cfg(target_os = "none")]
        {
            let mut sin = stdin();
            let mut sout = stdout();
            let mut buf = [0u8; 8192];
            loop {
                match sin.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if sout.write(&buf[..n]).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
        #[cfg(not(target_os = "none"))]
        {
            let mut sin = io::stdin();
            let mut sout = io::stdout();
            let mut buf = [0u8; 8192];
            loop {
                match sin.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if sout.write(&buf[..n]).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    } else {
        for path in &argv[1..] {
            if path == "-" {
                #[cfg(target_os = "none")]
                {
                    let mut sin = stdin();
                    let mut sout = stdout();
                    let mut buf = [0u8; 8192];
                    loop {
                        match sin.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => {
                                if sout.write(&buf[..n]).is_err() {
                                    break;
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
                #[cfg(not(target_os = "none"))]
                {
                    let mut sin = io::stdin();
                    let mut sout = io::stdout();
                    let mut buf = [0u8; 8192];
                    loop {
                        match sin.read(&mut buf) {
                            Ok(0) => break,
                            Ok(n) => {
                                if sout.write(&buf[..n]).is_err() {
                                    break;
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
            } else {
                #[cfg(target_os = "none")]
                {
                    match File::open(path) {
                        Ok(mut file) => {
                            let mut sout = stdout();
                            let mut buf = [0u8; 8192];
                            loop {
                                match file.read(&mut buf) {
                                    Ok(0) => break,
                                    Ok(n) => {
                                        if sout.write(&buf[..n]).is_err() {
                                            break;
                                        }
                                    }
                                    Err(_) => {
                                        eprintln!("cat: error reading file");
                                        break;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            eprintln!("cat: error opening file");
                            exit(1);
                        }
                    }
                }
                #[cfg(not(target_os = "none"))]
                {
                    match std::fs::File::open(path) {
                        Ok(mut file) => {
                            let mut sout = io::stdout();
                            let mut buf = [0u8; 8192];
                            loop {
                                match file.read(&mut buf) {
                                    Ok(0) => break,
                                    Ok(n) => {
                                        if sout.write(&buf[..n]).is_err() {
                                            break;
                                        }
                                    }
                                    Err(_) => {
                                        eprintln!("cat: error reading file");
                                        break;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            eprintln!("cat: error opening file");
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
    }
}