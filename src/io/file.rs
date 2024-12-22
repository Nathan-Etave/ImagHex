use image::{ImageBuffer, ImageError, Rgba};
use png::{BitDepth, ColorType, Compression, Encoder, Writer};
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;

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
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    output_path: &str,
) -> Result<(), ImageError> {
    let file: File = File::create(output_path).unwrap();
    let ref mut buf_writer: BufWriter<File> = BufWriter::new(file);

    let mut encoder: Encoder<&mut BufWriter<File>> = Encoder::new(buf_writer, width, height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    encoder.set_compression(Compression::Default);
    let mut writer: Writer<&mut BufWriter<File>> = encoder.write_header().unwrap();

    writer.write_image_data(&image).unwrap();

    Ok(())
}

pub fn get_file_name(file_path: &str) -> String {
    let path: &Path = Path::new(file_path);
    let file_stem: &str = path.file_stem().unwrap().to_str().unwrap();
    file_stem.to_string()
}

pub fn get_file_extension(file_path: &str) -> String {
    let path: &Path = Path::new(file_path);
    let extension: &str = path.extension().unwrap().to_str().unwrap();
    extension.to_string()
}
