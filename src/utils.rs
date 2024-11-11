use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn create_color_grid(width: usize, height: usize) -> Vec<Vec<Rgb>> {
  let colors: Vec<Rgb> = (0..(width * height))
      .map(|_| random_rgb())
      .collect();

  // Reshape the flat vector into a 2D Vec<Vec<Rgb>>
  colors
      .chunks(width)
      .map(|row| row.to_vec())
      .collect()
}

pub fn random_rgb() -> Rgb {
    let mut rng = rand::thread_rng();
    Rgb {
        r: rng.gen_range(0..=255),
        g: rng.gen_range(0..=255),
        b: rng.gen_range(0..=255),
    }
}

use std::f32;

pub fn diff(rgb1: Vec<u8>, rgb2: Vec<u8>) -> f32 {
    let dr:f32 = rgb1[0] as f32 - rgb2[0] as f32 ;
    let dg = rgb1[1]as f32 - rgb2[1]as f32 ;
    let db = rgb1[2]as f32 - rgb2[2]as f32 ;
    (dr * dr + dg * dg + db * db).sqrt()
}





