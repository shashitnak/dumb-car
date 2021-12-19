extern crate crypto;

use std::fs::File;
use std::io;
use std::io::prelude::*;

const CHUNK: usize = 0x800;

pub fn chunk(file_name: &str) -> io::Result<()> {
    println!("Chunking {}...", file_name);
    let mut file = File::open(file_name)?;
    let mut buffer = [0u8; CHUNK];
    let mut chunks = 0;

    loop {
        let bytes = file.read(&mut buffer)?;
        if bytes == 0 {
            break;
        }
        let chunk_name = format!("chunk_{}.chnk", chunks + 1);
        chunks += 1;
        let mut chunk = File::create(chunk_name)?;
        chunk.write(&buffer[..bytes])?;
    }
    println!("{} Chunked!", file_name);
    Ok(())
}

pub fn unchunk(file_name: &str) -> io::Result<()> {
    println!("Unchunking {}...", file_name);
    let mut file = File::create(file_name)?;
    let mut buffer = [0u8; CHUNK];
    let mut chunks = 0;

    loop {
        let chunk_name = format!("chunk_{}.chnk", chunks + 1);
        chunks += 1;
        let mut chunk = match File::open(chunk_name) {
            Ok(a) => a,
            Err(_) => break,
        };
        let bytes = chunk.read(&mut buffer)?;
        file.write(&buffer[..bytes])?;
    }
    println!("{} Created!", file_name);
    Ok(())
}
