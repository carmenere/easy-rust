# 4 ways to read a file in Rust
## Read the entire file as a `String`
```rust
fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
```

<br>

## Read the entire file as a `Vector`
```rust
fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}
```

<br>

## Read a text file `line by line`
```rust
use std::{io::{BufReader, BufRead as _}, fs::File};

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(read_file_line_by_line("/tmp/testfile")?)
}
```

<br>

## Read a file by chunks
```rust
fn read_file_buffer(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;

    loop {
        let read_count = file.read(&mut buffer)?;
        do_something(&buffer[..read_count]);

        if read_count != BUFFER_LEN {
            break;
        }
    }
    Ok(())
}
```