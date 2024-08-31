use image::RgbImage;

use crate::io::file;

pub fn decode_file(file_path: &str, output_location: &Option<String>) -> Result<(), String> {
    let image: RgbImage = match image::open(file_path) {
        Ok(image) => image.to_rgb8(),
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let mut header_length: Vec<u8> = image
        .pixels()
        .take(3)
        .map(|pixel| pixel.0)
        .flatten()
        .collect();
    header_length = header_length[..8].to_vec();

    let header_length: usize = usize::from_le_bytes(header_length.try_into().unwrap());

    let pixels_needed: usize = (header_length + 2) / 3;
    let header: Vec<u8> = image
        .pixels()
        .skip(3)
        .take(pixels_needed)
        .map(|pixel| pixel.0)
        .flatten()
        .collect();

    let file_name = match extract_string_from_header(&header, 0) {
        Some(name) => name,
        None => {
            return Err("Could not extract file name from header".to_string());
        }
    };

    let file_extension = match extract_string_from_header(&header, 1) {
        Some(extension) => extension,
        None => {
            return Err("Could not extract file extension from header".to_string());
        }
    };

    let mut buffer_length: Vec<u8> = header.split(|&x| x == 0).skip(2).next().unwrap().to_vec();
    while buffer_length.len() < 8 {
        buffer_length.push(0);
    }
    let buffer_length: usize = usize::from_le_bytes(buffer_length.try_into().unwrap());

    let buffer: Vec<u8> = image
        .pixels()
        .skip(3 + pixels_needed - 1)
        .take(buffer_length)
        .map(|pixel| pixel.0)
        .flatten()
        .collect();

    let buffer: Vec<u8> = buffer
        .iter()
        .rev()
        .skip_while(|&x| x == &0)
        .collect::<Vec<&u8>>()
        .iter()
        .rev()
        .map(|&x| *x)
        .collect();

    let output_path = format!(
        "{}/{}.{}",
        output_location.clone().unwrap_or(String::from(".")),
        file_name,
        file_extension
    );

    if let Err(e) = file::write_file(&output_path, &buffer) {
        return Err(e.to_string());
    }

    Ok(())
}

fn extract_string_from_header(header: &[u8], index: usize) -> Option<String> {
    let mut header_iter = header.split(|&x| x == 0);
    let header_string = header_iter.nth(index)?;
    let header_string = String::from_utf8(header_string.to_vec()).ok()?;
    Some(header_string)
}
