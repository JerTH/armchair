//#![recursion_limit = "1024"]
//#![feature(trace_macros)]
//trace_macros!(true);

#[macro_use]
pub mod decode;
pub mod instructions;
pub mod memory;
pub mod loader;
pub mod processor;
