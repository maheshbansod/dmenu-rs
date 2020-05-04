use std::os::raw::{c_uchar};
use std::mem::{self, MaybeUninit};
use std::ptr;
use std::ffi::{CString};
use libc::{c_int, c_uint, c_char};

pub enum Schemes { SchemeNorm, SchemeSel, SchemeOut, SchemeLast }
pub enum Clrs    { ColFg, ColBg }

pub static COLORS: [[[u8; 8]; 2]; Schemes::SchemeLast as usize] =  {
    /*     fg         bg       */
    let mut arr: [[[u8; 8]; 2]; Schemes::SchemeLast as usize] = [[[0; 8]; 2]; Schemes::SchemeLast as usize]; // init is optimized out
    arr[Schemes::SchemeNorm as usize] = [*b"#bbbbbb\0", *b"#222222\0"];
    arr[Schemes::SchemeSel  as usize] = [*b"#eeeeee\0", *b"#005577\0"];
    arr[Schemes::SchemeOut  as usize] = [*b"#000000\0", *b"#00ffff\0"];
    arr
};

pub struct Config {
    pub lines: c_uint,
    pub topbar: c_int,
    pub prompt: String, // TODO: str or string?
}

impl Default for Config {
    fn default() -> Self {
	Self{
	    lines: 0,
	    topbar: 1,
	    prompt: "Prompt:".to_string(),//ptr::null(), // TODO: make null when working
	}
    }
}
