use crate::io::file::{get_file_extension, get_file_name};

#[derive(Debug)]
pub struct Header {
    pub(crate) file_name: String,
    pub(crate) file_extension: String,
    pub(crate) buffer_length: usize,
}

impl Header {
    pub fn new(file_path: &str, buffer_length: usize) -> Self {
        Self {
            file_name: get_file_name(file_path),
            file_extension: get_file_extension(file_path),
            buffer_length,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let header_size: usize = std::mem::size_of::<usize>()
            + self.file_name.len()
            + 1
            + self.file_extension.len()
            + 1
            + std::mem::size_of::<usize>();
        let mut header: Vec<u8> = Vec::with_capacity(header_size);

        header.extend_from_slice(&(header_size - std::mem::size_of::<usize>()).to_le_bytes());

        header.extend_from_slice(self.file_name.as_bytes());
        header.push(0);
        header.extend_from_slice(self.file_extension.as_bytes());
        header.push(0);
        header.extend_from_slice(&self.buffer_length.to_le_bytes());

        header
    }

    pub fn from_bytes(header_bytes: &Vec<u8>) -> Result<Self, String> {
        let file_name = extract_string_from_header_bytes(header_bytes, 0)
            .ok_or("Could not extract file name from header".to_string())?;

        let file_extension = extract_string_from_header_bytes(header_bytes, 1)
            .ok_or("Could not extract file extension from header".to_string())?;

        let mut buffer_length = header_bytes
            .split(|&x| x == 0)
            .skip(2)
            .next()
            .unwrap()
            .to_vec();

        while buffer_length.len() < 8 {
            buffer_length.push(0);
        }

        let buffer_length = usize::from_le_bytes(buffer_length.try_into().unwrap());

        Ok(Self {
            file_name,
            file_extension,
            buffer_length,
        })
    }
}

pub fn extract_string_from_header_bytes(header: &Vec<u8>, index: usize) -> Option<String> {
    let mut header_iter = header.split(|&x| x == 0);
    let header_string: &[u8] = header_iter.nth(index)?;
    let header_string: String = String::from_utf8(header_string.to_vec()).ok()?;
    Some(header_string)
}
