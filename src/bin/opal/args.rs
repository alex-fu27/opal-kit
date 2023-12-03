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
use clap::{Args, Parser, Subcommand};

use std::fs;
use std::io;
use regex::Regex;

#[derive(Parser, Debug)]
pub struct Lock {
    #[arg(short, long, default_value="0")]
    range: u8,
}

#[derive(Subcommand, Debug)]
pub enum MBRCommand {
    On,
    Off,
    Enable,
    Disable,
}

#[derive(Parser, Debug)]
pub struct MBR {
    #[command(subcommand)]
    command: MBRCommand
}

#[derive(Subcommand, Debug)]
pub enum Command {
    List,
    Lock(Lock),
    Unlock(Lock),
    Save(Lock),
    MBR(MBR),
}

#[derive(Args, Debug)]
pub struct Common {
    #[arg()]
    pub drives: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(version,
          about = "Manipulate TCG OPAL 2.0 compliant drives"
)]
pub struct Full {
    #[command(flatten)]
    pub common_args: Common,

    #[command(subcommand)]
    pub command: Command,
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

pub fn parse() -> Full {
    let mut args = Full::parse();
    if args.common_args.drives.is_empty() {
        args.common_args.drives = list_devices().unwrap();
    }
    args
}
