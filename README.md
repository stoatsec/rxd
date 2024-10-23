
# RXD: Hexdump Written in Rust

A terminal-based hexdump with configurations for memory usage and regex search. View the ASCII output of a target file alongside the binary or hexadecimal contents.


## Installation: Building from source

Currently the only way to use this project is by building from source with Cargo

```bash 
  git clone https://github.com/stoatsec/rxd.git
  cd rxd

  cargo build --release
```

If you want the program to be globally available as a command, you can move the binary into a directory in your shell's path
```bash
  sudo mv target/release/rxd /usr/bin/
```
    
## Usage / Examples

Specify the size of chunks to read the file in
```
rxd <FILEPATH> -c <chunk size>
```

Display in binary format instead of hexadecimal
```
rxd <FILEPATH> -b
```

Apply a regex search to the output (will highlight the raw/ascii output with blue)
```
rxd <FILEPATH> -p <regex pattern>
```
## Features

- memory efficient file reading
- regex searching output
- cool looking border box ;)

### Planned Features

- built-in grep functionality
- buffering lines instead of printing directly for faster output
- fix regex search blindspot
