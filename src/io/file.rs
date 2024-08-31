use image::{ImageBuffer, Rgb};
use png;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

pub fn read_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file: File = File::open(file_path)?;
    let mut buffer: Vec<u8> = vec![];

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn write_file(file_path: &str, buffer: &Vec<u8>) -> io::Result<()> {
    let mut file: File = File::create(file_path)?;

    file.write_all(buffer)?;

    Ok(())
}

pub fn write_compressed_png(
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    output_path: &str,
) -> io::Result<()> {
    let file: File = File::create(output_path).unwrap();
    let ref mut buf_writer: BufWriter<File> = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buf_writer, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Default);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&image).unwrap();

    Ok(())
}
