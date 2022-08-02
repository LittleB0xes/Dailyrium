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
    current_fg_color: Color,
    current_bg_color: Color,
    layers: Vec<Layer>,
}

impl Terminal {
    /// Create a new terminal with size and layers
    pub fn new(width: u32, height: u32, cell_width: u32, cell_height: u32, number_of_layer: u32) -> Terminal {
        let mut all_layers: Vec<Layer> = Vec::new();
        for layer in 0..number_of_layer {
            let mut new_layer = Layer {width, height, data: Vec::new()};
            for index in 0..(width * height) {
                let mut new_cell = Cell::new(index % width, index / width, '.' as u16);
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
            layer_max: number_of_layer,
            current_layer: 0,
            current_fg_color: RED,
            current_bg_color: BLACK,
            layers: all_layers,
        }
    }


    pub fn layer(&mut self, layer: u32) {
        self.current_layer = layer;
    }

    pub fn fg_color(&mut self, color: Color) {
        self.current_fg_color = color;
    }

    pub fn bg_color(&mut self, color: Color) {
        self.current_bg_color = color;
    }

    pub fn fill(&mut self, glyph: u16) {
        self.fill_layer(self.current_layer, glyph);
    }
    pub fn fill_area(&mut self, glyph: u16, xo: u32, yo: u32, width: u32, height: u32) {
        self.fill_layer_area(self.current_layer, glyph, xo, yo, width, height);
    }
    pub fn put_layer(&mut self, layer_index: u32, x: u32, y: u32, glyph: u16) {
        self.put_layer_ex(layer_index, x, y, glyph, self.current_fg_color, self.current_bg_color);
    }
    pub fn put(&mut self, x: u32, y: u32, glyph: u16) {
        self.put_layer_ex(self.current_layer, x, y, glyph,self.current_fg_color, self.current_bg_color);   
    }

    pub fn put_ex(&mut self,x: u32, y: u32, glyph: u16, fg_color: Color, bg_color: Color) {
        self.put_layer_ex(self.current_layer, x, y, glyph, fg_color, bg_color);
    }

    pub fn put_layer_ex(&mut self,layer_index: u32, x: u32, y: u32, glyph: u16, fg_color: Color, bg_color: Color) {
        let index = x + y * self.width;
        self.layers[layer_index as usize].data[index as usize].glyph = glyph;
        self.layers[layer_index as usize].data[index as usize].bg_color = bg_color;
        self.layers[layer_index as usize].data[index as usize].fg_color = fg_color;
    
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
            self.put_layer(layer_index, x, y, glyph);
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
        let bg_draw_param = DrawTextureParams {
            dest_size: Some(Vec2::new(16.0, 16.0)),
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
        draw_texture_ex(texture, (cell.x * 16) as f32, (cell.y * 16) as f32, cell.bg_color, bg_draw_param );
        draw_texture_ex(texture, (cell.x * 16) as f32, (cell.y * 16) as f32, cell.fg_color, draw_param );
    }
}