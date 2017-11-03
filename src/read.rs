use std::error::Error;
use std::fs::File;
use std::io::Read;

use RangeBytes;

const BUF_LENGTH: usize = 1024;
const NULL_BYTE: u8 = 0u8;

fn check_for_null_bytes(buffer: &[u8; BUF_LENGTH]) -> Vec<RangeBytes> {
    let mut processing = false;
    let mut beg: u64 = 0;
    let mut range_bytes = Vec::new();
    for (i, byte) in buffer.iter().enumerate() {
        if byte == &NULL_BYTE && ! processing {
            processing = true;
            beg = i as u64;
        }
        if byte != &NULL_BYTE && processing {
            processing = false;
            range_bytes.push(RangeBytes(beg, i as u64));
            beg = 0;
        }
    }
    // If the parsing does not finished, push the last data structure
    if processing {
        range_bytes.push(RangeBytes(beg, BUF_LENGTH as u64))
    }
    range_bytes
}

pub struct InputChunkReader {
    zero_bytes_chunks: Vec<RangeBytes>,
}

impl InputChunkReader {

    pub fn read(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut buf = [0u8; BUF_LENGTH];
        loop {
            let buf_reader = file.read(&mut buf);
            match buf_reader {
                Ok(0) => break,
                Ok(len_reader) => {
                    // zero_bytes_chunks.push
                },
                Err(err) => {
                    error!(&format!("Got an error looking for chunks: {}", err.description()));
                    break;
                },
            }
        }
    }
    
    pub fn new() -> InputChunkReader {
        InputChunkReader { zero_bytes_chunks: Vec::new() }
    }

}