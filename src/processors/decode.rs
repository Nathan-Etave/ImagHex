use crate::io::file;
use crate::processors::common::Header;
use image::RgbaImage;

struct ImageDecoder {
    header: Header,
    buffer: Vec<u8>,
}

impl ImageDecoder {
    fn new(file_path: &str) -> Result<Self, String> {
        let image = image::open(file_path)
            .map_err(|e| e.to_string())?
            .to_rgba8();

        let header_length: Vec<u8> = image.pixels().take(2).flat_map(|pixel| pixel.0).collect();
        let header_length = usize::from_le_bytes(header_length.try_into().unwrap());

        let pixels_needed = (header_length + 3) / 4;
        let header_bytes: Vec<u8> = image
            .pixels()
            .skip(2)
            .take(pixels_needed)
            .flat_map(|pixel| pixel.0)
            .take(header_length)
            .collect();

        let header = Header::from_bytes(&header_bytes)?;
        let buffer = Self::extract_buffer(&image, pixels_needed, header.buffer_length);

        Ok(Self { header, buffer })
    }

    fn extract_buffer(image: &RgbaImage, header_pixels: usize, buffer_length: usize) -> Vec<u8> {
        let buffer: Vec<u8> = image
            .pixels()
            .skip(2 + header_pixels - 1)
            .flat_map(|pixel| pixel.0)
            .take(buffer_length)
            .collect();

        buffer
            .iter()
            .rev()
            .skip_while(|&x| x == &0)
            .collect::<Vec<&u8>>()
            .iter()
            .rev()
            .map(|&x| *x)
            .collect()
    }

    fn save(&self, output_location: &Option<String>) -> Result<(), String> {
        let output_path = format!(
            "{}/{}.{}",
            output_location.clone().unwrap_or(String::from(".")),
            self.header.file_name,
            self.header.file_extension
        );

        file::write_file(&output_path, &self.buffer).map_err(|e| e.to_string())
    }
}

pub fn decode_file(file_path: &str, output_location: &Option<String>) -> Result<(), String> {
    let decoder = ImageDecoder::new(file_path)?;
    decoder.save(output_location)
}
