use macroquad::prelude::*;

//#![allow(dead_code)]


pub struct Cell {
    x: u32,
    y: u32,
    glyph: u16,
}

impl Cell {
    pub fn new(x: u32, y: u32, glyph: u16) -> Cell {
        Cell {
            x,
            y,
            glyph,
        }
    }

    pub fn draw(&self, texture: Texture2D) {
        let draw_param = DrawTextureParams {
            dest_size: Some(Vec2::new(16.0, 16.0)),
            source: Some(Rect::new(16.0 * (self.glyph % 16) as f32, 16.0 * (self.glyph / 16) as f32, 16.0, 16.0)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(texture, (self.x * 16) as f32, (self.y * 16) as f32, WHITE, draw_param );
    }
}

struct Layer {
    width: u32,
    height: u32,
    data: Vec<Cell>,
}



pub struct Terminal {
    width: u32,
    height: u32,
    cell_width: u32,
    cell_height: u32,
    layer_max: u32,
    layers: Vec<Layer>,
}

impl Terminal {
    pub fn new(width: u32, height: u32, cell_width: u32, cell_height: u32, number_of_layer: u32) -> Terminal {
        let all_layers: Vec<Layer> = Vec::new();
        Terminal {
            width,
            height,
            cell_width,
            cell_height,
            layer_max: number_of_layer,
            layers: all_layers,
        }
    }
}