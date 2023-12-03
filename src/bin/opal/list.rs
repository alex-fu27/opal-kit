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

use opal_kit::Disk;

pub fn subcommand(devices: &Vec<String>) {
    for d in devices {
        let disk = Disk::open(&d).unwrap();
        match disk.get_status() {
            Ok(status) => println!("{} {}", d, status),
            Err(_) => println!("{} Unsupported", d),
        }
    }
}

