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
use super::args::{Hash, HashVariant};
use opal_kit::hashing::{dta_sedutil_hash, ladar_sedutil_hash};
use opal_kit::Disk;
use rpassword::*;

pub fn subcommand(args: &Hash) {
    let devices = args.common_args.get_drives();
    if devices.len() != 1 {
        panic!("specify exactly one device to hash the password for");
    }
    let device = &devices[0];
    let id = Disk::open(device).expect("could not access disk").get_id();

    let passwd = prompt_password(format!("enter password for {}: ", device))
        .expect("could not read password");

    let hash = match args.variant {
		HashVariant::Sedutil => dta_sedutil_hash(&passwd.as_bytes(), &id.serial_number),
		HashVariant::SedutilSHA512 => ladar_sedutil_hash(&passwd.as_bytes(), &id.serial_number),
		HashVariant::Argon2id => panic!("I need to research proper parameters for the Argon2id algorithm before fixing the hash algorithm here forever."),
	 };

    println!("{}", hex::encode(hash));
}
