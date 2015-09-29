use std::io::prelude::*;
use std::fs::File;
use std::vec::*;

pub struct RGB {
  pub r: u8,
  pub g: u8,
  pub b: u8
}
 
pub struct PPM {
  height: usize,
  width: usize,
  data: Vec<u8>
}
 
impl PPM {
  pub fn new(height: usize, width: usize) -> PPM {
    let size = 3 * height * width;
    let buffer = vec![0u8; size];
    PPM {height: height, width: width, data: buffer}
  }
 
  fn buffer_size(&self) -> usize {
    3 * self.height * self.width
  }
 
  fn get_offset(&self, x: usize, y: usize) -> Option<usize> {
    let offset = (y * self.width * 3) + (x * 3);
    if offset < self.buffer_size() {
      Some(offset)
    } else {
      None
    }
  }
 
  // pub fn get_pixel(&self, x: usize, y: usize) -> Option<RGB> {
  //   match self.get_offset(x, y) {
  //     Some(offset) => {
  //       let r = self.data[offset];
  //       let g = self.data[offset + 1];
  //       let b = self.data[offset + 2];
  //       Some(RGB {r: r, g: g, b: b})
  //     },
  //     None => None
  //   }
  // }
 
  pub fn set_pixel(&mut self, x: usize, y: usize, color: RGB) -> bool {
    match self.get_offset(x, y) {
      Some(offset) => {
        self.data[offset] = color.r;
        self.data[offset + 1] = color.g;
        self.data[offset + 2] = color.b;
        true
      },
      None => false
    }
  }
 
  pub fn write_file(&self, filename: &str) -> bool {
    let mut f = File::create(&filename).unwrap();
    let header = format!("P6 {} {} 255\n", self.width, self.height);
    f.write(header.as_bytes());
    f.write(&self.data);
    true
  }
 
}