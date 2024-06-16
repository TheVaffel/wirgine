use num::traits::Zero;

use std::mem::size_of;
use std::ptr::from_mut;
use std::slice::from_raw_parts;

pub struct Image<PixelType: Zero + Copy> {
    width: u32,
    height: u32,
    data: Vec<PixelType>,
}

impl<PixelType: Zero + Copy> Image<PixelType> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![PixelType::zero(); (width * height) as usize],
        }
    }

    pub fn data_ptr(&mut self) -> *mut u8 {
        self.data[..].as_mut_ptr() as *mut u8
    }

    pub fn data_slice(&mut self) -> &[u8] {
        let type_size = size_of::<PixelType>();
        unsafe {
            from_raw_parts(
                self.data[..].as_ptr() as *const u8,
                (self.width * self.height) as usize * type_size,
            )
        }
    }
}

impl Image<u32> {
    pub fn rgb_shuffle(&mut self) -> () {
        unsafe {
            for x in 0..self.width {
                for y in 0..self.height {
                    let pixel = (from_mut(&mut self.data[(y * self.width + x) as usize]) as *mut _
                        as *mut [u8; 4]);
                    let r = (*pixel)[0];
                    let b = (*pixel)[2];
                    (*pixel)[0] = b;
                    (*pixel)[2] = r;
                }
            }
        }
    }
}
