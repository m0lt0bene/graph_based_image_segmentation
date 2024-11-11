use image::{ImageBuffer, Pixel};
use imageproc::filter::gaussian_blur_f32;
use std::f32;

use crate::{segment_graph::segment_graph, union_find::Edge, utils::{create_color_grid, diff, random_rgb, Rgb}};

pub fn segment_image(im: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>> , sigma: f32, c: f32, min_size: usize) -> Vec<Vec<Rgb>> {
    let width = im.width();
    let height = im.height();

    let mut filtered: ImageBuffer<image::Rgb<u8>, Vec<u8>> = gaussian_blur_f32(im, sigma);
    let mut channels:Vec<Vec<u8>> = filtered.enumerate_pixels_mut().map(|(x,y,rgb)| vec![rgb.channels()[0],rgb.channels()[1],rgb.channels()[2]]).collect();
    // smooth each color channel  
    let mut edges: Vec<Edge> = Vec::with_capacity(width as usize * height as usize  * 4);
    println!("{}",channels.len());
    let channels_matrix: Vec<_> = channels.chunks(width as usize).collect(); 
    println!("{}",channels_matrix.len());
    println!("{}",channels_matrix[1].len());

    let height = channels_matrix.len();
    let width = channels_matrix[1].len();

    for y in 0..height {
      for x in 0..width {
          if x < width - 1 {
              let w = diff(channels_matrix[y][x].clone(),channels_matrix[y][x+1].clone());
              edges.push(Edge::new(w,y * width + x, y * width + x+1));          
          };
          if y < height - 1 {
            let w = diff(channels_matrix[y][x].clone(),channels_matrix[y+1][x].clone());
            edges.push(Edge::new(w,y * width + x, (y+1) * width + x));    
          }
          if x < width - 1 && y < height - 1 {
            let w = diff(channels_matrix[y][x].clone(),channels_matrix[y+1][x+1].clone());
            edges.push(Edge::new(w,y * width + x, (y+1) * width + x+1));    
          }
          if x < width - 1 && y > 0 {
            let w = diff(channels_matrix[y][x].clone(),channels_matrix[y-1][x+1].clone());
            edges.push(Edge::new(w,y * width + x, (y-1) * width + x+1));    
          }
      } 
    }
    let mut u = segment_graph(width * height, edges.len(), &mut edges, c);    
    // post process small components
    for edge in &edges {
        let a = u.find(edge.a);
        let b = u.find(edge.b);
        if a != b && (u.size(a) < min_size.try_into().unwrap() || u.size(b) < min_size.try_into().unwrap()) {
            u.join(a, b);
        } 
    }
    let num_ccs = u.num_sets();
    //println!("edges {:?}",edges);
    let mut output = create_color_grid(width, height);
    // pick random colors for each component
    let colors: Vec<Rgb> = (0..(width * height)).map(|_| random_rgb()).collect();
    
    for y in 0..height {
        for x in 0..width {
            let comp = u.find(y * width + x);
            output[y][x] =  colors[comp];
        }
    }  

    output 
}

