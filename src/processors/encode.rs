use crate::io::file;
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub fn encode_file(file_path: &str, output_location: &Option<String>) -> Result<(), String> {
    let file_name: String = get_file_name(file_path);
    let file_buffer: Vec<u8> = file::read_file(file_path).unwrap();

    let buffer_length: usize = file_buffer.len();
    let header: Vec<u8> = create_header(file_path, &file_name, buffer_length);
    let total_length: usize = header.len() + buffer_length;
    let pixel_count: usize = (total_length + 2) / 3;

    let width: u32 = (pixel_count as f64).sqrt().ceil() as u32;
    let height: u32 = ((pixel_count as f64) / (width as f64)).ceil() as u32;

    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (i, chunk) in header.chunks(3).enumerate() {
        let x: u32 = (i as u32) % width;
        let y: u32 = (i as u32) / width;
        let r = chunk.get(0).copied().unwrap_or(0);
        let g = chunk.get(1).copied().unwrap_or(0);
        let b = chunk.get(2).copied().unwrap_or(0);
        image.put_pixel(x, y, Rgb([r, g, b]));
    }

    for i in 0..buffer_length / 3 {
        let x: u32 = ((i + header.len() / 3) as u32) % width;
        let y: u32 = ((i + header.len() / 3) as u32) / width;
        let r = file_buffer.get(i * 3).copied().unwrap_or(0);
        let g = file_buffer.get(i * 3 + 1).copied().unwrap_or(0);
        let b = file_buffer.get(i * 3 + 2).copied().unwrap_or(0);
        image.put_pixel(x, y, Rgb([r, g, b]));
    }

    for i in (header.len() / 3 + buffer_length / 3)..(width * height) as usize {
        let x: u32 = (i as u32) % width;
        let y: u32 = (i as u32) / width;
        image.put_pixel(x, y, Rgb([0, 0, 0]));
    }

    let output_path = format!(
        "{}/{}.png",
        output_location.clone().unwrap_or(String::from(".")),
        file_name
    );

    if let Err(e) = file::write_compressed_png(image, width, height, &output_path) {
        return Err(e.to_string());
    }

    Ok(())
}

fn create_header(file_path: &str, file_name: &str, buffer_length: usize) -> Vec<u8> {
    let file_extension: String = get_file_extension(file_path);

    let mut header: Vec<u8> = vec![];

    header.extend_from_slice(file_name.as_bytes());

    header.push(0);

    header.extend_from_slice(file_extension.as_bytes());

    header.push(0);

    header.extend_from_slice(&buffer_length.to_le_bytes());

    let header_length: usize = header.len();
    let mut new_header = Vec::with_capacity(header_length.to_le_bytes().len() + header.len());
    new_header.extend_from_slice(&header_length.to_le_bytes());
    new_header.push(0);
    new_header.extend_from_slice(&header);

    new_header
}

fn get_file_name(file_path: &str) -> String {
    let path: &Path = Path::new(file_path);
    let file_stem: &str = path.file_stem().unwrap().to_str().unwrap();
    file_stem.to_string()
}

fn get_file_extension(file_path: &str) -> String {
    let path: &Path = Path::new(file_path);
    let extension: &str = path.extension().unwrap().to_str().unwrap();
    extension.to_string()
}
