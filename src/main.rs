mod chunker;

use std::env::args;
use std::io;

fn main() -> io::Result<()> {
    for arg in args().skip(1) {
        chunker::chunk(&arg)?;
        chunker::unchunk(&format!("unchnkd_{}", arg))?;
    }
    Ok(())
}
