extern crate ansi_term;
#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate hyper;
extern crate hyper_native_tls;
extern crate pbr;
extern crate rayon;

use std::sync::{Arc, Mutex};

#[macro_use]
pub mod logs;

pub mod authorization;
// pub mod bench;
pub mod cargo_helper;
pub mod client;
pub mod contentlength;
pub mod download;
pub mod filesize;
pub mod http_version;
pub mod protocol;
pub mod response;
pub mod util;
pub mod write;

/// Represents a number of bytes, as `u64`.
pub type Bytes = u64;
/// Represents a 'chunk', which is just a piece of bytes.
type Chunk = Vec<u8>;
/// Represents a list of chunks
pub type Chunks = Vec<Chunk>;
/// Represents a range between two Bytes types
#[derive(Debug, PartialEq)]
struct RangeBytes(Bytes, Bytes);
/// Represents a shared mutable reference of chunks
pub type SChunks = Arc<Mutex<Chunks>>;
/// Represents an URL
pub type URL<'a> = &'a str;
