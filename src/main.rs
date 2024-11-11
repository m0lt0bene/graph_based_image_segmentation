use std::path::Path;

use graph_based_image_segmentation::segment_image::segment_image;
use image::Pixel;

fn main() {
    let from = "/home/bh/Documents/graph_based_image_segmentation/data/tree.jpg";
    let mut im: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::open(Path::new(&from)).unwrap().into_rgb8();
    // Iterate over the coordinates and pixels of the image

    let output = segment_image(&mut im, 0.5, 500.0, 50);
    let imgx = output[1].len()as u32 ;
    let imgy = output.len() as u32;
    

    
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = output[y as usize][x as usize].r;
        let g = output[y as usize][x as usize].g;
        let b = output[y as usize][x as usize].b;
        *pixel = image::Rgb([r, g, b]);
    }
    

    
    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("result.png").unwrap();
}
