#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

#[cfg(target_os = "none")]
use libposix::{File, Read, Write, stdin, stdout, args, exit, println, print, eprintln};
#[cfg(target_os = "none")]
use alloc::{string::String, vec::Vec};

#[cfg(not(target_os = "none"))]
use std::io::{self, Read};
#[cfg(not(target_os = "none"))]
use std::env;

#[cfg(target_os = "none")]
struct Args;

#[cfg(target_os = "none")]
impl Args {
    fn from_stack() -> Vec<String> {
        libposix::args()
    }
}

#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
fn _start(_args: Args) {
    let argv = Args::from_stack();
    run(&argv);
    exit(0);
}

#[cfg(not(target_os = "none"))]
fn main() {
    let argv: Vec<String> = env::args().collect();
    run(&argv);
}

fn run(argv: &[String]) {
    let mut i = 1;

    while i < argv.len() && argv[i].starts_with('-') && argv[i] != "--" {
        if argv[i] == "--" { i += 1; break; }
        eprintln!("od: invalid option -- '{}'", argv[i]);
        #[cfg(target_os = "none")]
        exit(1);
        #[cfg(not(target_os = "none"))]
        std::process::exit(1);
    }

    let files: Vec<String> = argv[i..].to_vec();
    let mut data = Vec::new();

    if files.is_empty() {
        #[cfg(target_os = "none")]
        {
            let mut sin = stdin();
            let mut buf = [0u8; 4096];
            loop {
                match sin.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => data.extend_from_slice(&buf[..n]),
                    Err(_) => break,
                }
            }
        }
        #[cfg(not(target_os = "none"))]
        {
            let mut sin = io::stdin();
            let mut buf = [0u8; 4096];
            loop {
                match sin.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => data.extend_from_slice(&buf[..n]),
                    Err(_) => break,
                }
            }
        }
    } else {
        #[cfg(target_os = "none")]
        {
            let mut f = File::open(&files[0]).unwrap_or_else(|_| {
                eprintln!("od: {}", files[0]);
                exit(1);
            });
            let mut buf = [0u8; 4096];
            loop {
                match f.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => data.extend_from_slice(&buf[..n]),
                    Err(_) => break,
                }
            }
        }
        #[cfg(not(target_os = "none"))]
        {
            let mut f = std::fs::File::open(&files[0]).unwrap_or_else(|_| {
                eprintln!("od: {}", files[0]);
                std::process::exit(1);
            });
            let mut buf = [0u8; 4096];
            loop {
                match f.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => data.extend_from_slice(&buf[..n]),
                    Err(_) => break,
                }
            }
        }
    }

    let bytes_per_group = 2;
    let groups_per_line = 8;

    for (chunk_idx, chunk) in data.chunks(bytes_per_group * groups_per_line).enumerate() {
        let addr = chunk_idx * bytes_per_group * groups_per_line;
        print!("{:07o}", addr);

        for group in chunk.chunks(bytes_per_group) {
            let val = if group.len() >= 2 {
                (group[0] as u16) | ((group[1] as u16) << 8)
            } else {
                group[0] as u16
            };
            print!(" {:06o}", val);
        }
        println!();
    }

    let total = data.len();
    if total > 0 {
        println!("{:07o}", total);
    }
}