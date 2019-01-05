extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
pub struct Rgb24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Rgb24Wrap(Rgb24);

impl Rgb24Wrap {
    pub fn new(c: Rgb24) -> Self {
        Rgb24Wrap(c)
    }
}

#[wasm_bindgen]
impl Rgb24Wrap {
    pub fn r(&self) -> u8 {
        self.0.r
    }
    pub fn g(&self) -> u8 {
        self.0.g
    }
    pub fn b(&self) -> u8 {
        self.0.b
    }
}
