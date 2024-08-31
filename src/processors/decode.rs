use image::RgbImage;

use crate::io::file;

pub fn decode_file(file_path: &str, output_location: Option<String>) {
    let image: RgbImage = image::open(file_path).unwrap().to_rgb8();

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

    let file_name: String =
        String::from_utf8(header.split(|&x| x == 0).next().unwrap().to_vec()).unwrap();

    let file_extension: String =
        String::from_utf8(header.split(|&x| x == 0).skip(1).next().unwrap().to_vec()).unwrap();

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
        output_location.unwrap_or(String::from(".")),
        file_name,
        file_extension
    );

    file::write_file(&output_path, &buffer).unwrap();
}
