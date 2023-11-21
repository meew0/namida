#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]

extern crate libc;
pub mod client;
pub mod common;
pub mod datagram;
pub mod extc;
pub mod server;
pub mod util;

pub fn main() {
    unsafe {
        match std::env::args().skip(1).next().as_deref() {
            Some("client") => {
                client::main::main_0(0, std::ptr::null_mut());
            }
            Some("server") => {
                server::main::main_0(0, std::ptr::null_mut());
            }
            Some(_) | None => {
                println!("For now, run either `namida client` or `namida server`.");
            }
        }
    }
}
