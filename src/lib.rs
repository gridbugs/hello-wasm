extern crate other;
extern crate wasm_bindgen;
use other::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Style {
    pub bold: bool,
    pub underline: bool,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Cell {
    pub code_point: char,
    pub style: Style,
    pub fg_colour: Rgb24Wrap,
    pub bg_colour: Rgb24Wrap,
}

#[wasm_bindgen]
pub struct Grid {
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cells: vec![
                Cell {
                    code_point: 'Q',
                    style: Style {
                        bold: true,
                        underline: false,
                    },
                    fg_colour: Rgb24Wrap::new(Rgb24 {
                        r: 0,
                        g: 127,
                        b: 255,
                    }),
                    bg_colour: Rgb24Wrap::new(Rgb24 {
                        r: 255,
                        g: 127,
                        b: 255,
                    }),
                },
                Cell {
                    code_point: 'u',
                    style: Style {
                        bold: false,
                        underline: false,
                    },
                    fg_colour: Rgb24Wrap::new(Rgb24 {
                        r: 255,
                        g: 255,
                        b: 255,
                    }),
                    bg_colour: Rgb24Wrap::new(Rgb24 { r: 0, g: 0, b: 0 }),
                },
            ],
        }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn get(&self, index: usize) -> Cell {
        self.cells[index]
    }
}
