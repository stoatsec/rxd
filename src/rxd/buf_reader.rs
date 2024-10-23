use regex::Regex;
use std::fs;
use std::io::{Read, Seek, SeekFrom};

pub struct ChunkedReader {
    file: fs::File,
    pub chunk_size: usize,
    consumed_bytes: usize,
}

impl ChunkedReader {
    pub fn new(file: fs::File, chunk_size: usize) -> Self {
        Self {
            file,
            chunk_size,
            consumed_bytes: 0,
        }
    }

    pub fn read_next_chunk(&mut self) -> Result<Option<Vec<u8>>, std::io::Error> {
        // checks for if we've read the entire file already
        if self.file.metadata()?.len() == self.consumed_bytes as u64 {
            return Ok(None);
        }

        self.file
            .seek(SeekFrom::Start(self.consumed_bytes as u64))?;

        let mut buffer = vec![0; self.chunk_size];
        let reader = std::io::BufReader::new(&self.file);

        match reader.take(self.chunk_size as u64).read_to_end(&mut buffer) {
            Ok(n) => {
                self.consumed_bytes += n;

                if n == self.chunk_size {
                    return Ok(Some(buffer[n..].to_vec()));
                }

                // resize buffer if not full
                let sub_buf: Vec<u8> = buffer[buffer.len() - n..].to_vec();
                Ok(Some(sub_buf))
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn peek_next_n_bytes(
        &mut self,
        num_bytes: usize,
    ) -> Result<Option<Vec<u8>>, std::io::Error> {
        // checks for if we've read the entire file already
        if self.file.metadata()?.len() == self.consumed_bytes as u64 {
            return Ok(None);
        }

        self.file
            .seek(SeekFrom::Start(self.consumed_bytes as u64))?;

        let mut buffer = vec![0; num_bytes];
        let reader = std::io::BufReader::new(&self.file);

        match reader.take(num_bytes as u64).read_to_end(&mut buffer) {
            Ok(n) => {
                self.consumed_bytes += n;

                if n == num_bytes {
                    return Ok(Some(buffer[n..].to_vec()));
                }

                // resize buffer if not full
                let sub_buf: Vec<u8> = buffer[buffer.len() - n..].to_vec();
                Ok(Some(sub_buf))
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

// regex search returns indices that are applicable for both the byte vec and char vec (same size)
pub fn regex_search(binary: &Vec<u8>, re: &Regex) -> Vec<(usize, usize)> {
    let chars = pair_ascii(binary);
    let string = chars.clone().into_iter().collect::<String>();
    let mut result: Vec<(usize, usize)> = vec![];

    for match_ in re.find_iter(&string) {
        result.push((match_.start(), match_.end()));
    }

    result
}

pub fn pair_ascii(binary: &Vec<u8>) -> Vec<char> {
    binary.iter().map(|&byte| byte as char).collect()
}