use std::mem::MaybeUninit;
use libc::{c_int, c_uint};

pub enum Schemes { SchemeNorm, SchemeSel, SchemeOut, SchemeLast }
pub enum Clrs    { ColFg, ColBg }
use Schemes::*;

pub static COLORS: [[[u8; 8]; 2]; SchemeLast as usize] =  {
    /*     fg         bg       */
    let mut arr: [[[u8; 8]; 2]; SchemeLast as usize] = [[[0; 8]; 2]; SchemeLast as usize]; // init is optimized out
    arr[SchemeNorm as usize] = [*b"#bbbbbb\0", *b"#222222\0"];
    arr[SchemeSel  as usize] = [*b"#eeeeee\0", *b"#005577\0"];
    arr[SchemeOut  as usize] = [*b"#000000\0", *b"#00ffff\0"];
    arr
};

#[derive(Debug)]
pub struct Config {
    pub lines: c_uint,
    pub topbar: c_int,
    pub prompt: String, // TODO: str or string?
    pub promptw: c_int,
    pub default_font: String,
    pub fast: bool,
}

impl Default for Config {
    fn default() -> Self {
	unsafe {
	    Self{
		lines: 0,
		topbar: 1,
		prompt: "Prompt".to_string(),//ptr::null(), // TODO: make null when working
		promptw: MaybeUninit::uninit().assume_init(),
		default_font: "monospace:size=10\0".to_string(),
		fast: false,
	    }
	}
    }
}
