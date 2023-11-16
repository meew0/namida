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
pub mod src {
pub mod client {
pub mod command;
pub mod config;
pub mod io;
pub mod main;
pub mod network;
pub mod protocol;
pub mod ring;
pub mod transcript;
} // mod client
pub mod common {
pub mod common;
pub mod error;
pub mod md5;
} // mod common
pub mod server {
pub mod config;
pub mod io;
pub mod log;
pub mod main;
pub mod network;
pub mod protocol;
pub mod transcript;
} // mod server
pub mod util {
pub mod fusereadtest;
pub mod readtest;
pub mod writetest;
} // mod util
} // mod src
