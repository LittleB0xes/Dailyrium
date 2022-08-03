#![allow(dead_code)]

use macroquad::prelude::*;


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
    data: Vec<Cell>,
    default_bg_color: Color,
    default_fg_color: Color,
}



pub struct Terminal {
    width: u32,
    height: u32,
    cell_width: u32,
    cell_height: u32,
    scale: u32,
    layer_max: u32,
    current_layer: u32,
    layers: Vec<Layer>,
}

impl Terminal {
    /// Create a new terminal with size and layers
    pub fn new(width: u32, height: u32, cell_width: u32, cell_height: u32, scale: u32, number_of_layer: u32) -> Terminal {
        let mut all_layers: Vec<Layer> = Vec::new();
        for layer in 0..number_of_layer {
            let mut new_layer = Layer { data: Vec::new(), default_fg_color: WHITE, default_bg_color: BLACK};
            new_layer.default_fg_color = WHITE;

            // Set transparency for default value for upper layers
            if layer != 0 {
                new_layer.default_bg_color.a = 0.0;
            }

            for index in 0..(width * height) {
                let mut new_cell = Cell::new(index % width, index / width, ' ' as u16);
                new_cell.fg_color = RED;
                
                // First Layer have a default background color
                if layer == 0 {

                    new_cell.bg_color = BLACK;
                }
                // Other Layers have transparency background by default
                else {
                    new_cell.bg_color = Color::new(0.0, 0.0, 0.0, 0.0);
                }

                new_layer.data.push(new_cell);
            }
            all_layers.push(new_layer);
        }
        Terminal {
            width,
            height,
            cell_width,
            cell_height,
            scale,
            layer_max: number_of_layer,
            current_layer: 0,
            layers: all_layers,
        }
    }

    /// Set the current active layer
    pub fn layer(&mut self, layer: u32) {
        self.current_layer = layer;
    }

    /// Set the foreground color of the current layer
    pub fn fg_color(&mut self, color: Color) {
        self.layers[self.current_layer as usize].default_fg_color = color;
    }

    /// Set the background color of the current layer
    pub fn bg_color(&mut self, color: Color) {
        self.layers[self.current_layer as usize].default_bg_color = color;
    }

    /// Fill the current layer with a glyph (u16)
    pub fn fill(&mut self, glyph: u16) {
        self.fill_layer(self.current_layer, glyph);
    }

    /// Fill an area of the current layer with a glyph
    /// Warning : out of bound checking yet (comming soon)
    pub fn fill_area(&mut self, glyph: u16, xo: u32, yo: u32, width: u32, height: u32) {
        self.fill_layer_area(self.current_layer, glyph, xo, yo, width, height);
    }

    /// Put a glyph in a specific place in a specific layer
    pub fn put_layer(&mut self, layer_index: u32, x: u32, y: u32, glyph: u16) {
        let fg_color = self.layers[self.current_layer as usize].default_fg_color;
        let bg_color = self.layers[self.current_layer as usize].default_bg_color;
        self.put_layer_ex(layer_index, x, y, glyph, fg_color, bg_color);
    }
    
    /// Put a glyph in a specific place in the current layer
    pub fn put(&mut self, x: u32, y: u32, glyph: u16) {
        self.put_layer(self.current_layer, x, y, glyph);
    }

    /// Put a glyph in a specific place in the current layer, with given color
    pub fn put_ex(&mut self,x: u32, y: u32, glyph: u16, fg_color: Color, bg_color: Color) {
        self.put_layer_ex(self.current_layer, x, y, glyph, fg_color, bg_color);
    }

    /// Put a glyph in a specific place in a specific layer, with given color
    pub fn put_layer_ex(&mut self,layer_index: u32, x: u32, y: u32, glyph: u16, fg_color: Color, bg_color: Color) {
        let index = x + y * self.width;
        self.layers[layer_index as usize].data[index as usize].glyph = glyph;
        self.layers[layer_index as usize].data[index as usize].bg_color = bg_color;
        self.layers[layer_index as usize].data[index as usize].fg_color = fg_color;
    
    }
    
    /// Fill a specific layer with a glyph
    pub fn fill_layer(&mut self, layer_index: u32, glyph: u16) {
        for index in 0..self.width * self.height {
            self.layers[layer_index as usize].data[index as usize].glyph = glyph;
        }
    }

    /// Fill a specific layer area with a glyph
    pub fn fill_layer_area(&mut self,layer_index: u32, glyph: u16, xo: u32, yo: u32, width: u32, height: u32) {
        for relative_index in 0..width * height {
            let x = xo + relative_index % width;
            let y = yo + relative_index / width;
            self.put_layer(layer_index, x, y, glyph);
        } 
    }


    // All Terminal's layers rendering
    pub fn render(&self, texture: Texture2D) {
        for layer_index in 0..self.layer_max {
            for index in 0..self.width * self.height {
                let cell_to_draw = self.layers[layer_index as usize].data[index as usize];
                self.draw_cell(texture, cell_to_draw);
            }
        }
    }

    // draw a cell (foreground and background)
    fn draw_cell(&self, texture: Texture2D, cell: Cell) {
        let bg_draw_param = DrawTextureParams {
            dest_size: Some(Vec2::new((self.scale * self.cell_width) as f32, (self.scale * self.cell_height) as f32)),
            source: Some(Rect::new(
                self.cell_width as f32 * 11.0, //11 13
                self.cell_height as f32 * 13.0,
                self.cell_width as f32,
                self.cell_height as f32)),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        let draw_param = DrawTextureParams {
            dest_size: Some(Vec2::new((self.scale * self.cell_width) as f32, (self.scale * self.cell_height) as f32)),
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
        draw_texture_ex(texture, (cell.x * self.cell_width * self.scale) as f32, (cell.y * self.cell_height * self.scale) as f32, cell.bg_color, bg_draw_param );
        draw_texture_ex(texture, (cell.x * self.cell_width * self.scale) as f32, (cell.y * self.cell_height * self.scale) as f32, cell.fg_color, draw_param );
    }

    /// Print a string in position
    pub fn print(&mut self, x: u32, y: u32, string: String) {
        let fg_color = self.layers[self.current_layer as usize].default_fg_color;
        let bg_color = self.layers[self.current_layer as usize].default_bg_color;
        self.print_ex(x, y, string, fg_color, bg_color)
    }

    /// Print a string in a position with given colors
    pub fn print_ex(&mut self, x: u32, y: u32, string: String, fg_color: Color, bg_color: Color) {
        for (i, letter) in string.chars().enumerate() {
            self.put_ex(x + i as u32, y, letter as u16, fg_color, bg_color);
        }
    }
}