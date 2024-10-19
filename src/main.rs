use macroquad::prelude::*;
use miniquad::window::screen_size;
use shadowgame::{player::Player, shader::Shader};

fn transform_shader_pos(v: &Vec3, ratio: f32) -> Vec3 {
    vec3(v.x, v.y / ratio, v.z)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Shadow Game".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: 1024,
        window_height: 576,
        ..Default::default()
    }
}

struct Light {
    pos_rad: Vec3,
    color: Vec3,
}
const MAX_LIGHTS: usize = 4;

#[macroquad::main(window_conf())]
async fn main() {
    let mut uniforms = vec![
        UniformDesc::new("pos_rad", UniformType::Float3),
        UniformDesc::new("dims", UniformType::Float2),
    ];
    uniforms.reserve(MAX_LIGHTS * 2);
    for i in 0..MAX_LIGHTS {
        uniforms.push(UniformDesc::new(
            &format!("lights[{}].pos_rad", i),
            UniformType::Float3,
        ));
        uniforms.push(UniformDesc::new(
            &format!("lights[{}].color", i),
            UniformType::Float3,
        ));
    }

    let shader = Shader::new("shadowblob.vert", "shadowblob.frag");
    let material = load_material(
        shader.to_source(),
        MaterialParams {
            uniforms: uniforms,
            ..Default::default()
        },
    )
    .unwrap();
    material.set_uniform("dims", screen_size());

    let target = render_target(screen_width() as u32, screen_height() as u32);
    target.texture.set_filter(FilterMode::Nearest);

    let ratio = screen_width() / screen_height();
    let mut player = Player::new(vec2(0.0, 0.25), vec2(0.1, 0.1), vec2(0.5, -0.5));
    let ground = (-1.0, 0.4, 2.0, 0.2);
    let mut lights: Vec<Light> = vec![
        Light {
            pos_rad: vec3(0.5, 0.25, 0.1),
            color: vec3(0.9, 0.9, 0.9),
        },
        Light {
            pos_rad: vec3(0.9, 0.2, 0.0),
            color: vec3(0.0, 0.0, 0.0),
        },
        Light {
            pos_rad: vec3(0.2, 0.8, 0.0),
            color: vec3(0.0, 0.0, 0.0),
        },
        Light {
            pos_rad: vec3(0.0, 0.0, 0.0),
            color: vec3(0.0, 0.0, 0.0),
        },
    ];

    let mut camera = Camera2D {
        zoom: vec2(1.0, screen_width() / screen_height()),
        target: Vec2::ZERO,
        render_target: Some(target.clone()),
        ..Default::default()
    };

    loop {
        set_camera(&camera);

        clear_background(DARKGRAY);

        draw_rectangle(ground.0, ground.1, ground.2, ground.3, WHITE);
        player.draw();

        set_default_camera();
        clear_background(WHITE);

        gl_use_material(&material);
        for (i, light) in lights.iter_mut().enumerate() {
            material.set_uniform(
                &format!("lights[{}].pos_rad", i),
                transform_shader_pos(&light.pos_rad, ratio),
            );
            material.set_uniform(&format!("lights[{}].color", i), light.color);
        }
        draw_texture_ex(
            &target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        let mut move_dir = Vec2::ZERO;
        if is_key_down(KeyCode::A) {
            move_dir.x -= 0.01;
        }

        if is_key_down(KeyCode::D) {
            move_dir.x += 0.01;
        }

        player.movement(move_dir);
        camera.target.x += move_dir.x;

        next_frame().await
    }
}
