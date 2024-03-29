use image::{DynamicImage, GenericImageView, Rgba, GenericImage, Pixel};
use crate::wave::{Wave,Region};

// Given a wave return the corresponding image
pub fn render_wave(wave: &Wave, output_path: String,tile_set_path: &String){
    let (x,y) = wave.size;
    let mut strips = Vec::new();
    // Joins all of the images in a row of the wave vertically then joins all of those images horizontally giving the final result
    for i in 0..x {
        let mut strip = Vec::new();
        for j in 0..y {
            strip.push(render_region(wave.regions.get(&(i,j)).unwrap(),tile_set_path));
        }
        strips.push(join_images_vertically(strip));
    }
    join_images(strips, output_path);
}

// Given a region return the corresponding image
pub fn render_region(region: &Region,tile_set_path: &String) -> DynamicImage {
    blend_images(region.superposition.keys().map(|k| format!("{}{}",tile_set_path,k)).collect())
}

// Joins all of the given images vertically
pub fn join_images_vertically(images: Vec<DynamicImage>) -> DynamicImage {
    // Find the total width of all images
    let total_height: u32 = images.iter().map(|img| img.height()).sum();
    
    // Find the maximum height of all images
    let max_width = images.iter().map(|img| img.width()).max().unwrap();
    
    // Create a new image buffer to hold the result
    let mut result = DynamicImage::new_rgb8(max_width, total_height);

    // Copy each image into the result image buffer
    let mut y_offset = 0;
    for img in images {
        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                result.put_pixel(x, y + y_offset, img.get_pixel(x, y));
            }
        }
        y_offset += height;
    }

    // Save the result image to the specified output path
    result
}

// Joins all of the images given horizontally
pub fn join_images(images: Vec<DynamicImage>, output_path: String) {

    // Join images vertically
    let max_height = images.iter().map(|img| img.height()).max().unwrap();
    let total_width = images.iter().map(|img| img.width()).sum();
    let mut result = DynamicImage::new_rgb8(total_width, max_height);
    let mut x_offset = 0;
    for img in images {
        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                result.put_pixel(x + x_offset, y, img.get_pixel(x, y));
            }
        }
        x_offset += width;
    }

    // Save the result image to the specified output path
    dbg!(output_path.clone());
    result.save(output_path).unwrap();
}

// If a region has multiple possible tile outcomes than those tiles are blended together into one tile
pub fn blend_images(images: Vec<String>) -> DynamicImage {
    // Load all images and store them in a vector
    let mut images_vec: Vec<DynamicImage> = Vec::new();
    for path in images {
        //dbg!(path.clone());
        let img = image::open(path).unwrap();
        images_vec.push(img);
    }

    // Calculate weight of each image
    let weight = 1.0 / (images_vec.len() as f32);

    // Find the maximum dimensions of all images
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;
    for img in &images_vec {
        let (width, height) = img.dimensions();
        if width > max_width {
            max_width = width;
        }
        if height > max_height {
            max_height = height;
        }
    }

    // Create a new image buffer to hold the result
    let mut result = DynamicImage::new_rgba8(max_width, max_height);

    // Blend each image into the result image buffer
    for img in &images_vec {
        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let pixel1 = result.get_pixel(x, y).to_rgba();
                let pixel2 = img.get_pixel(x, y).to_rgba();
                let blended_pixel = blend_pixels(pixel1, pixel2, weight);
                result.put_pixel(x, y, blended_pixel);
            }
        }
    }

    result
}

// Blend two pixels together given their weight
fn blend_pixels(pixel1: Rgba<u8>, pixel2: Rgba<u8>, weight: f32) -> Rgba<u8> {
    let r = ((pixel1[0] as f32 * (1.0 - weight)) + (pixel2[0] as f32 * weight)) as u8;
    let g = ((pixel1[1] as f32 * (1.0 - weight)) + (pixel2[1] as f32 * weight)) as u8;
    let b = ((pixel1[2] as f32 * (1.0 - weight)) + (pixel2[2] as f32 * weight)) as u8;
    let a = 255;

    Rgba([r, g, b, a])
}