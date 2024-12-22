use crate::io::file::{read_file, write_compressed_png};
use crate::processors::common::Header;
use image::{ImageBuffer, Rgba};

struct ImageEncoder {
    header: Header,
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImageEncoder {
    fn new(file_path: &str, data: Vec<u8>) -> Self {
        let header: Header = Header::new(file_path, data.len());
        let image_size: usize = header.to_bytes().len() + data.len();
        let pixel_count: usize = (image_size + 3) / 4;
        let width: u32 = (pixel_count as f64).sqrt().ceil() as u32;
        let height: u32 = width;

        Self {
            header,
            data,
            width,
            height,
        }
    }

    fn encode(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(self.width, self.height);

        let header_bytes: Vec<u8> = self.header.to_bytes();
        self.write_bytes_to_image(&mut image, &header_bytes, 0);
        self.write_bytes_to_image(&mut image, &self.data, header_bytes.len());
        self.fill_remaining_pixels(&mut image, header_bytes.len() + self.data.len());

        image
    }

    fn write_bytes_to_image(
        &self,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        bytes: &Vec<u8>,
        offset: usize,
    ) {
        for (i, chunk) in bytes.chunks(4).enumerate() {
            let x: u32 = ((i + offset / 4) as u32) % self.width;
            let y: u32 = ((i + offset / 4) as u32) / self.width;
            let pixel: Rgba<u8> = Rgba([
                chunk.get(0).copied().unwrap_or(0),
                chunk.get(1).copied().unwrap_or(0),
                chunk.get(2).copied().unwrap_or(0),
                chunk.get(3).copied().unwrap_or(0),
            ]);
            image.put_pixel(x, y, pixel);
        }
    }

    fn fill_remaining_pixels(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, offset: usize) {
        for i in (offset / 4)..(self.width * self.height) as usize {
            let x: u32 = (i as u32) % self.width;
            let y: u32 = (i as u32) / self.width;
            let current_pixel = image.get_pixel(x, y);
            if current_pixel.0 == [0, 0, 0, 0] {
                image.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }
}

pub fn encode_file(file_path: &str, output_location: &Option<String>) -> Result<(), String> {
    let file_buffer: Vec<u8> = read_file(file_path).unwrap();
    let image_encoder: ImageEncoder = ImageEncoder::new(file_path, file_buffer);
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = image_encoder.encode();

    let output_path: String = format!(
        "{}/{}.png",
        output_location.clone().unwrap_or(String::from(".")),
        image_encoder.header.file_name
    );

    write_compressed_png(
        image,
        image_encoder.width,
        image_encoder.height,
        &output_path,
    )
    .map_err(|e| e.to_string())
}
