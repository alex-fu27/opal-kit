/* This file is part of opal-kit.
 * 
 * opal-kit is free software: you can redistribute it and/or modify it under the terms of the GNU
 * General Public License as published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 * 
 * opal-kit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along with opal-kit. If not,
 * see <https://www.gnu.org/licenses/>.
 */
use clap::{Parser, Subcommand};
use std::fs;
use std::io;
use regex::Regex;

use opal_kit::{Disk, Status};

#[derive(Parser, Debug)]
struct LockArgs {
    #[arg(short, long, default_value="0")]
    range: u8,
}

#[derive(Subcommand, Debug)]
enum MBRCommand {
    On,
    Off,
    Enable,
    Disable,
}

#[derive(Parser, Debug)]
struct MBRArgs {
    #[command(subcommand)]
    command: MBRCommand
}

#[derive(Subcommand, Debug)]
enum Command {
    List,
    Lock(LockArgs),
    Unlock(LockArgs),
    Save(LockArgs),
    MBR(MBRArgs),
}

#[derive(Parser, Debug)]
#[command(version,
          about = "Manipulate TCG OPAL 2.0 compliant drives"
)]
struct Args {
    #[arg()]
    drives: Vec<String>,

    #[command(subcommand)]
    command: Command,
}

fn list_devices() -> io::Result<Vec<String>> {
    let re = Regex::new(r"^/dev/((nvme\d\d*)n\d\d*|sd\w)$").unwrap();
    let all = fs::read_dir("/dev")?;
    let as_string = |e: io::Result<fs::DirEntry>| String::from(e.unwrap().path().to_str().unwrap());
    Ok(all.filter(|e| match e {
        Ok(e) => re.is_match(e.path().to_str().unwrap()),
        Err(_) => false
    }).map(as_string).collect())
}

fn subcommand_list(devices: &Vec<String>) {
    for d in devices {
        let disk = Disk::open(&d).unwrap();
        match disk.get_status() {
            Ok(status) => println!("{} {}", d, status),
            Err(_) => println!("{} Unsupported", d),
        }
    }
}

fn main() {
    let args = Args::parse();
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .try_init();
    let devs = if args.drives.is_empty() {
        list_devices().unwrap()
    } else {
        args.drives.clone()
    };
    match &args.command {
        Command::List => subcommand_list(&devs),
        _ => println!("{:?}", &args),
    }
}
