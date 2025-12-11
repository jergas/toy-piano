use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let input_path = "/Users/jergas/.gemini/antigravity/brain/78baf3c7-7468-42cc-a505-33f66c5a8320/uploaded_image_1765455115930.png";
    let output_path = "assets/abstract-soundwave-icon.png";

    println!("Processsing image: {}", input_path);

    let img = image::open(input_path).expect("Failed to open input image");
    let (width, height) = img.dimensions();

    let mut new_img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let a = pixel[3];

        // Simple threshold for white background
        // The uploaded image seems to have a clean white bg, but might have jpeg artifacts if it went through compression.
        // We'll be slightly lenient (val > 240)
        let is_white = r > 240 && g > 240 && b > 240;

        if is_white {
            new_img.put_pixel(x, y, Rgba([0, 0, 0, 0])); // Transparent
        } else {
            new_img.put_pixel(x, y, Rgba([r, g, b, a])); // Keep original
        }
    }

    new_img.save(output_path).expect("Failed to save output image");
    println!("Saved transparent icon to: {}", output_path);
}
