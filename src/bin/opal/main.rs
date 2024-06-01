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
mod args;
mod list;

use args::Command;

fn main() {
    let args = args::parse();
    let common_args = args.common_args;
    let command = args.command;

    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .try_init()
        .unwrap();

    match &command {
        Command::List => list::subcommand(&common_args.drives),
        _ => panic!("not implemented {:?} {:?}", &common_args, &command),
    }
}
