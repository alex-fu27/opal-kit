#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;

impl nvme_id {
	pub fn get_serial_number(&self) -> String {
		String::from_utf8((&self.serial_number).to_vec())
			.unwrap()
			.trim()
			.into()
	}

	pub fn get_model_number(&self) -> String {
		String::from_utf8((&self.model_number).to_vec())
			.unwrap()
			.trim()
			.into()
	}
}
