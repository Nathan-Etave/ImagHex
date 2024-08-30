use std::io::{self, Read, Write};
use std::fs::File;

pub fn read_file(file_path: &str) -> io::Result<Vec<u8>> {
  let mut file: File = File::open(file_path)?;
  let mut buffer: Vec<u8> = vec![];

  file.read_to_end(&mut buffer)?;

  Ok(buffer)
}

pub fn write_file(file_name: &str, file_path: &str, buffer: &Vec<u8>) -> io::Result<()> {
  let mut file: File = File::create(format!("{}/{}", file_path, file_name))?;

  file.write_all(buffer)?;

  Ok(())
}