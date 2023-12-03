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
use std::fs::File;
use std::io;
use std::mem::MaybeUninit;
use rustix::fd::AsRawFd;

use linux_sed_opal_sys::*;

#[derive(Debug)]
pub struct Status {
    supported: bool, //1
    locking_supported: bool, // 2
    locking_enabled: bool, // 4
    locked: bool, // 8
    mbr_enabled: bool, // 16
    mbr_done: bool, // 32
}

impl From<&opal_status> for Status {
    fn from(s: &opal_status) -> Self {
        Self {
            supported: (s.flags & OPAL_FL_SUPPORTED) != 0,
            locking_supported: (s.flags & OPAL_FL_SUPPORTED) != 0,
            locking_enabled: (s.flags & OPAL_FL_LOCKING_ENABLED) != 0,
            locked: (s.flags & OPAL_FL_LOCKED) != 0,
            mbr_enabled: (s.flags & OPAL_FL_MBR_ENABLED) != 0,
            mbr_done: (s.flags & OPAL_FL_MBR_DONE) != 0,
        }
    }
}

impl From<opal_status> for Status {
    fn from(s: opal_status) -> Self {
        (&s).into()
    }
}

#[derive(Debug)]
pub struct Disk {
    file: File,
}

impl Disk {
    pub fn open(path: &str) -> io::Result<Self> {
        Ok(Self {
            file: File::open(&path)?,
        })
    }

    fn get_raw_fd(&self) -> i32 {
        self.file.as_raw_fd()
    }

    pub fn get_status(&self) -> nix::Result<Status> {
        let mut status = MaybeUninit::<[opal_status; 1]>::uninit();
        unsafe {
            let err = ioc_opal_get_status(self.get_raw_fd(), &mut *status.as_mut_ptr())?;
            if err < 0 {
                panic!("hmm");
            }
            Ok(status.assume_init()[0].into())
        }
    }
}

