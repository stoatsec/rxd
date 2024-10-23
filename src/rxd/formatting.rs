use crate::colors::*;
use core::cmp;
use regex::Regex;
use std::collections::HashSet;

use super::buf_reader::ChunkedReader;
use super::buf_reader::{pair_ascii, regex_search};

// determines the amount of bytes/chars per line for each format
const BIN_WIDTH: u8 = 6;
const HEX_WIDTH: u8 = 16;

pub fn display_chunks(
    mut chunked_reader: ChunkedReader,
    regex: Option<Regex>,
    binaryfmt: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut count: usize = 0;
    while let Some(ref mut bytevec) = chunked_reader.read_next_chunk()? {
        let mut indices = vec![];
        let mut _match: bool = false;
        let format_count: usize = match binaryfmt {
            false => HEX_WIDTH,
            true => BIN_WIDTH,
        } as usize;

        // get the overflow -- remainder between the chunk size and fmt width
        let overflow = format_count.saturating_sub(bytevec.len() % format_count);
        if let Some(overflowvec) = chunked_reader.peek_next_n_bytes(overflow)? {
            // fill in gaps between chunks by reading from an overflow buffer
            bytevec.extend(overflowvec);
        } // if this is not triggered (None returned), the whole file has been read and padding can be applied

        let pair_ascii = pair_ascii(bytevec);
        if let Some(ref re) = regex {
            indices = regex_search(&bytevec, re);
        }

        for (index, chunk) in bytevec
            .chunks(format_count)
            .zip(pair_ascii.chunks(format_count))
            .enumerate()
        {
            let start_index = index * format_count;
            count += 1;

            print!("│ {GRAY}{:08x}{RESET} │ ", count * format_count);
            for (local_index, byte) in chunk.0.iter().enumerate() {
                let unprintable: bool = !printable(chunk.1[local_index]);
                let global_index = start_index + local_index;

                _match = indices
                    .iter()
                    .any(|&indices| global_index >= indices.0 && global_index < indices.1);

                match (_match, unprintable, binaryfmt) {
                    // binary format
                    (false, true, true) => print!("{RED}{:08b}{RESET} ", byte),
                    (false, false, true) => print!("{:08b} ", byte),
                    (true, true, true) => print!("{PURPLE}{:08b}{RESET} ", byte),
                    (true, false, true) => print!("{SKYBLUE}{:08b}{RESET} ", byte),

                    // hexadecimal format
                    (false, true, false) => print!("{RED}{:02x}{RESET} ", byte),
                    (false, false, false) => print!("{:02x} ", byte),
                    (true, true, false) => print!("{PURPLE}{:02x}{RESET} ", byte),
                    (true, false, false) => print!("{SKYBLUE}{:02x}{RESET} ", byte),
                }
            }

            // padding (binary)
            if chunk.0.len() < format_count {
                let pad_unit = format_count - chunk.0.len();
                let bytes_pad_count = if binaryfmt {
                    pad_unit * 9 // length of one binary chunk plus a space
                } else {
                    pad_unit * 3
                };

                print!("{}", " ".repeat(bytes_pad_count));
            }

            print!("│ ");

            for (local_index, mut char) in chunk.1.iter().enumerate() {
                let unprintable: bool = !printable(*char);
                let global_index = start_index + local_index;

                _match = indices
                    .iter()
                    .any(|&indices| global_index >= indices.0 && global_index < indices.1);

                if unprintable {
                    char = &'.';
                }

                match (_match, unprintable) {
                    (false, true) => print!("{RED}{BOLD}{char}{RESET}"),
                    (false, false) => print!("{char}"),
                    (true, true) => print!("{BOLD}{PURPLE}{char}{RESET}"),
                    (true, false) => print!("{BOLD}{SKYBLUE}{char}{RESET}"),
                }
            }

            // padding (ascii)
            if chunk.1.len() < format_count {
                let char_pad_count = format_count - chunk.1.len();
                print!("{}", " ".repeat(char_pad_count));
            };

            println!(" │");
        }
    }

    Ok(())
}

pub fn display_title(binaryfmt: bool, filename: String, filesize: usize) {
    let mut filename = filename;
    let filesize = format_byte_count(filesize);
    if filename.len() > 30 {
        filename = truncate_text(&filename, 30)
    }

    let format_count: usize = match binaryfmt {
        false => HEX_WIDTH,
        true => BIN_WIDTH,
    } as usize;

    // magic numbers, anyone?

    print!("┌");
    print!("{}", dashes(filename.len() + 2)); // 2 for spaces
    print!("┬");
    print!("{}", dashes(filesize.len() + 2));
    println!("┐");

    println!("│ {} │ {} │", filename, filesize);
    print!("├");

    match filename.len().cmp(&8) {
        // 8 for the digits in the line count
        cmp::Ordering::Less => {
            print!("{}", dashes(filename.len() + 2));
            print!("┴");

            match (filename.len() + filesize.len() + 3).cmp(&8) {
                // because we know file name is less than 8, we need to check the length of filename + filesize now
                std::cmp::Ordering::Less => {
                    print!("{}", dashes(filesize.len() + 2));
                    print!("┴");
                    print!("┬");
                    // wont be calculating the dashes between these two connectors, because only a one character file name with one digit for the file size would trigger this
                }
                std::cmp::Ordering::Equal => {
                    print!("{}", dashes(filesize.len() + 2));
                    print!("┼");
                }
                std::cmp::Ordering::Greater => {
                    print!("{}", dashes(7 - filename.len()));
                    print!("┬");
                    print!("{}", dashes((filename.len() + filesize.len()) - 6));
                    print!("┴");
                }
            }
        }
        cmp::Ordering::Equal => {
            print!("{}", dashes(10)); // 8 for the line count len, 2 for spaces
            print!("┼");
            print!("{}", dashes(filesize.len() + 2));
            print!("┴");
        }
        cmp::Ordering::Greater => {
            print!("{}", dashes(10)); // 8 for the line count len, 2 for spaces
            print!("┬");
            print!("{}", dashes(filename.len() - 9)); // 9 is the min size that still qualifies as greater
            print!("┴");
            print!("{}", dashes(filesize.len() + 2));
            print!("┴");
        }
    }

    let titlelen = filename.len() + filesize.len() + 7; // 7 for spaces & border

    let contentwidth = match binaryfmt {
        true => (format_count * 9) + 13,
        false => (format_count * 3) + 13,
    };

    print!("{}", dashes(contentwidth - titlelen));
    print!("┬");
    print!("{}", dashes(format_count + 2));
    println!("┐");
}

pub fn display_footer(binaryfmt: bool) {
    let format_count: usize = match binaryfmt {
        false => HEX_WIDTH,
        true => BIN_WIDTH,
    } as usize;

    let contentwidth = match binaryfmt {
        true => format_count * 9,
        false => format_count * 3,
    };

    print!("└");
    print!("{}", dashes(10));
    print!("┴");
    print!("{}", dashes(contentwidth + 1)); // add one for trailing space
    print!("┴");
    print!("{}", dashes(format_count + 2));
    print!("┘");
}

fn truncate_text(s: &str, length: usize) -> String {
    if s.len() <= length {
        return s.to_string();
    }

    let truncated = &s[s.len() - length..];
    format!("{}{}", "[...]", truncated)
}

fn dashes(length: usize) -> String {
    "─".repeat(length)
}

fn format_byte_count(bytes: usize) -> String {
    let units = vec![
        ("TB", 1_000_000_000_000),
        ("GB", 1_000_000_000),
        ("MB", 1_000_000),
        ("KB", 1_000),
        ("B", 1),
    ];

    for (unit_name, unit_value) in &units {
        if bytes >= *unit_value {
            let size_in_unit = bytes / unit_value;
            return format!("{}.{}", size_in_unit, unit_name);
        }
    }

    // default to bytes if no unit can be matched
    format!("{}.B", bytes)
}

pub fn printable(character: impl Into<char>) -> bool {
    let printable_chars: HashSet<char> = {
        let mut chars = HashSet::new();
        for i in 32..=126 {
            chars.insert(char::from_u32(i).unwrap());
        }
        chars
    };

    printable_chars.contains(&character.into())
}