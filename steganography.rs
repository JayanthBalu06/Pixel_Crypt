use image::{GenericImageView, RgbaImage};
use crate::utils::{message_to_bits, bits_to_message};

pub fn embed_message_in_image(input_path: &str, output_path: &str, secret_message: &str) -> std::io::Result<()> {
    let mut img = image::open(input_path).expect("Failed to open input image");
    let (width, height) = img.dimensions();
    let mut img_buffer = img.to_rgba8();

    let secret_bits = message_to_bits(secret_message);

    let mut bit_index = 0;
    for y in 0..height {
        for x in 0..width {
            let mut pixel = img_buffer.get_pixel(x, y);
            let mut rgba = pixel.0;

            for i in 0..4 {
                if bit_index < secret_bits.len() {
                    rgba[i] = (rgba[i] & !1) | secret_bits[bit_index];
                    bit_index += 1;
                }
            }

            img_buffer.put_pixel(x, y, image::Rgba(rgba));
        }
    }

    img_buffer.save(output_path).expect("Failed to save output image");
    Ok(())
}

pub fn extract_message_from_image(file_path: &str, message_length: usize) -> std::io::Result<String> {
    let img = image::open(file_path).expect("Failed to open input image");
    let (width, height) = img.dimensions();
    let img_buffer = img.to_rgba8();

    let mut secret_bits: Vec<u8> = Vec::new();
    let mut bit_index = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = img_buffer.get_pixel(x, y);
            let rgba = pixel.0;

            for i in 0..4 {
                if bit_index < message_length * 8 {
                    secret_bits.push(rgba[i] & 1);
                    bit_index += 1;
                }
            }
        }
    }

    Ok(bits_to_message(&secret_bits).unwrap_or_else(|| "Invalid message".to_string()))
}
