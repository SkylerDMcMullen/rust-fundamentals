use std::io::{BufRead, BufReader, BufWriter};
use std::io::{self, Write, Seek, SeekFrom};
use std::fs::OpenOptions;

fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(true)
        .open("non_existent_file.txt");
    let mut file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Cannot open: permission denied")
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    // scoped since the writer borrows the file variable
    // must be freed in order for the reader to later use the file
    {
        let mut writer = BufWriter::new(&file);
        println!("Enter a line to write:");
        let mut input = String::new();
        let res = io::stdin().read_line(&mut input);
        match res {
            Ok(_res) => {
                match writer.write_all(input.as_bytes()) {
                    Ok(_) => {
                        writer.flush().expect("failed to flush");
                    }
                    Err(error) => panic!("Error: {}", error)
                }
                
            },
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::InvalidData => {
                        panic!("Data encoding not UTF-8: {}", error);
                    }
                    _ => {
                        panic!("Error: {}", error);
                    }
                }
            }
        };
    }
    
    // Since the writer appeneded to the file, need to reset the file cursor 
    file.seek(SeekFrom::Start(0)).expect("failed to seek to start");
    println!("File contents: ");
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
