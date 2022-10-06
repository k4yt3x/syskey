use std::{io, io::Write, process};

use anyhow::Result;
use clap::Parser;
use syskey::{run, Config, VERSION};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the trunking system's ID
    #[arg(index = 1)]
    sysid: Option<String>,
}

fn parse() -> Result<Config> {
    let args = Args::parse();

    let mut interactive = false;

    // get system ID
    let sysid = {
        let long = {
            if let Some(sysid) = args.sysid {
                sysid
            }
            else {
                interactive = true;
                print!("Enter the SysID: ");
                io::stdout().flush()?;
                let mut sysid = String::new();
                io::stdin().read_line(&mut sysid)?;
                sysid.chars().filter(|&x| x.is_alphanumeric()).collect()
            }
        }
        .to_uppercase();

        // check for input validity by attempting to convert the hex string into u32
        u32::from_str_radix(&long, 16)?;

        // take only the last four chars
        if long.len() > 4 {
            format!("{:0>4}", &long[long.len() - 4..])
        }
        else {
            format!("{:0>4}", long)
        }
    }
    .to_owned();

    Ok(Config::new(sysid, interactive))
}

/// program entry point
fn main() {
    println!("MOTOROLA SYSTEM KEY GENERATION UTILITY");
    println!("VERSION {}; October 5, 2022", VERSION);
    println!("(C) Copyright K4YT3X. 2022.");
    println!("Made available under the ISC license.\n");

    // parse command line arguments into Config
    match parse() {
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
        Ok(config) => process::exit(match run(config) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("error: {}", e);
                1
            }
        }),
    }
}
