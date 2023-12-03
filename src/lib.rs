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
use std::fmt;
use rustix::fd::AsRawFd;

use linux_sed_opal_sys::*;

#[derive(Debug)]
pub struct Status {
    pub supported: bool, //1
    pub locking_supported: bool, // 2
    pub locking_enabled: bool, // 4
    pub locked: bool, // 8
    pub mbr_enabled: bool, // 16
    pub mbr_done: bool, // 32
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

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let supported = |flag| if flag { "supported" } else { "unsupported" };
        let enabled = |flag| if flag { "enabled" } else { "disabled" };
        let on = |flag| if flag { "on" } else { "off" };
        write!(f, "{}, locking {}, locking {}, locking {}, mbr {}, mbr {}",
            supported(self.supported),
            supported(self.locking_supported),
            enabled(self.locking_enabled),
            on(self.locked),
            enabled(self.mbr_enabled),
            on(!self.mbr_done))
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

