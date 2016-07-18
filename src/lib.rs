

extern crate libc;


mod append_buffer;

use libc::{c_int, c_char};
use std::mem::{forget, drop, transmute};
use std::vec::Vec;
use std::string::String;
use append_buffer::AppendBuffer;

fn c_to_rust_str(c_str: *const c_char, len: c_int) -> String {
	let mut ret = String::new();
	
	for i in 0..len {
		let c = unsafe {
			*(c_str.offset(i as isize)) as u8 as char
		};
		ret.push(c);
	}
	
	ret
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn ABUF_INIT() -> *mut AppendBuffer {
	Box::into_raw(Box::new(AppendBuffer::new()))
	
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn abFree(buff: *mut AppendBuffer) {
	unsafe {
		Box::from_raw(buff);
	}
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn abAppend(buff: *mut AppendBuffer, string: *const c_char, len: c_int) {
	unsafe {
		(*buff).append(c_to_rust_str(string, len))
	}
	
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn abLen(buff: *mut AppendBuffer) -> c_int {
	unsafe {
		(*buff).get_string().len() as c_int
	}
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn abStr(buff: *mut AppendBuffer) -> *mut c_char {
	unsafe {
		transmute::<*const u8, *mut c_char>((*buff).get_string().as_ptr())
	}
}



