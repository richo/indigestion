#![crate_name = "indigestion"]

#![license = "MIT"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(phase)]
#[phase(plugin, link)] extern crate log;

extern crate libc;
extern crate core;

pub mod proxy;
