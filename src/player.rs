use animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub dims: Vec2,
    pub offset: Vec2,
    pub vel: Vec2,
    pub dir: Vec2,
    pub idle_sprite: Texture2D,
    pub walking_sprite: Texture2D,
    pub anim: AnimatedSprite,
}

impl Player {
    pub fn new(
        pos: Vec2,
        dims: Vec2,
        offset: Vec2,
        idle_sprite: Texture2D,
        walking_sprite: Texture2D,
    ) -> Self {
        Player {
            pos,
            dims,
            offset,
            dir: vec2(1.0, 0.0),
            vel: vec2(0.0, 0.0),
            idle_sprite,
            walking_sprite,
            anim: AnimatedSprite::new(
                64,
                64,
                &[
                    Animation {
                        name: "idle".to_string(),
                        row: 0,
                        frames: 3,
                        fps: 6,
                    },
                    Animation {
                        name: "walking".to_string(),
                        row: 0,
                        frames: 3,
                        fps: 6,
                    },
                ],
                true,
            ),
        }
    }

    pub fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
        if vel != Vec2::ZERO {
            self.dir.x = if vel.x < 0.0 { -1.0 } else { 1.0 };
        }
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
    }

    pub fn draw(&mut self) {
        let frame = self.anim.frame();
        let tex: &Texture2D = if self.vel != Vec2::ZERO {
            &self.walking_sprite
        } else {
            &self.idle_sprite
        };
        draw_texture_ex(
            tex,
            self.pos.x - self.dims.x / 2.0,
            self.pos.y - self.dims.x / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.dims),
                source: Some(frame.source_rect),
                flip_x: self.dir.x < 0.0,
                ..Default::default()
            },
        );
        self.anim.update();
    }
}
