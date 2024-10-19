use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub dims: Vec2,
    pub offset: Vec2,
}

impl Player {
    pub fn new(pos: Vec2, dims: Vec2, offset: Vec2) -> Self {
        Player { pos, dims, offset }
    }

    pub fn movement(&mut self, dir: Vec2) {
        self.pos += dir;
    }

    pub fn draw(&self) {
        draw_rectangle_ex(
            self.pos.x,
            self.pos.y,
            self.dims.x,
            self.dims.y,
            DrawRectangleParams {
                color: RED,
                offset: self.offset,
                ..Default::default()
            },
        );
    }
}
