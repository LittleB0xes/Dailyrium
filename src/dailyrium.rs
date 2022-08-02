use macroquad::prelude::*;

//#![allow(dead_code)]

#[derive(Copy, Clone)]
struct Cell {
    x: u32,
    y: u32,
    glyph: u16,
    fg_color: Color,
    bg_color: Color,
}

impl Cell {
    fn new(x: u32, y: u32, glyph: u16) -> Cell {
        Cell {
            x,
            y,
            glyph,
            fg_color: Color::new(0.5, 0.5, 0.0, 1.0),
            bg_color: Color::new(0.0, 0.0, 0.0, 1.0),
        }
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
    current_layer: u32,
    layers: Vec<Layer>,
}

impl Terminal {
    pub fn new(width: u32, height: u32, cell_width: u32, cell_height: u32, number_of_layer: u32) -> Terminal {
        let mut all_layers: Vec<Layer> = Vec::new();
        for _layer in 0..number_of_layer {
            let mut new_layer = Layer {width, height, data: Vec::new()};
            for index in 0..(width * height) {
                let new_cell = Cell::new(index % width, index / width, '.' as u16);
                new_layer.data.push(new_cell);
            }
            all_layers.push(new_layer);
        }
        Terminal {
            width,
            height,
            cell_width,
            cell_height,
            layer_max: number_of_layer,
            current_layer: 0,
            layers: all_layers,
        }
    }

    pub fn set_layer(&mut self, layer: u32) {
        self.current_layer = layer;
    }

    pub fn fill_layer(&mut self, layer_index: u32, glyph: u16) {
        for index in 0..self.width * self.height {
            self.layers[layer_index as usize].data[index as usize].glyph = glyph;
        }
    }

    pub fn fill_layer_area(&mut self,layer_index: u32, glyph: u16, xo: u32, yo: u32, width: u32, height: u32) {
        for relative_index in 0..width * height {
            let x = xo + relative_index % width;
            let y = yo + relative_index / width;
            let index = x + y * self.width;
            self.layers[layer_index as usize].data[index as usize].glyph = glyph;
        } 
    }


    pub fn render(&self, texture: Texture2D) {
        for layer_index in 0..self.layer_max {
            for index in 0..self.width * self.height {
                let cell_to_draw = self.layers[layer_index as usize].data[index as usize];
                self.draw_cell(texture, cell_to_draw);
            }
        }
    }
    fn draw_cell(&self, texture: Texture2D, cell: Cell) {
        let draw_param = DrawTextureParams {
            dest_size: Some(Vec2::new(16.0, 16.0)),
            source: Some(Rect::new(
                (self.cell_width * (cell.glyph as u32 % 16)) as f32,
                (self.cell_height * (cell.glyph as u32 / 16)) as f32,
                self.cell_width as f32,
                self.cell_height as f32)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(texture, (cell.x * 16) as f32, (cell.y * 16) as f32, cell.fg_color, draw_param );
    }
}