use std::fs::File;
use std::path::Path;

use regex::Regex;

mod colors;
mod rxd;
mod rxd_clap;

use rxd::buf_reader::ChunkedReader;
use rxd::formatting::{display_chunks, display_footer, display_title};
use rxd_clap::parse_args;

fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    let path = Path::new(&args.file_path);
    let file = File::open(path)?;
    let binaryfmt = args.binary;

    let pattern = args.pattern;
    let mut chunk_size = args.chunks;

    let file_size = file.metadata()?.len() as usize;

    // no need to load a kb of memory for each chunk if the file can be read in one
    if file_size < chunk_size {
        chunk_size = file_size;
    }

    let chunked_reader = ChunkedReader::new(file, chunk_size);
    let mut regex = None;

    if let Some(pattern) = &pattern {
        let re = Regex::new(pattern)?;
        regex = Some(re);

        // todo!
        // - buffer output lines to make displaying faster
        // - overlap vec with size of regex query to remove regex search blindspots
        // - line grep search with adjacent line view
    };

    display_title(binaryfmt, args.file_path, file_size);
    display_chunks(chunked_reader, regex, binaryfmt)?;
    display_footer(binaryfmt);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match execute() {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}
