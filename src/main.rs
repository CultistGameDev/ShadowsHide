use macroquad::prelude::*;
use miniquad::window::screen_size;
use shadowgame::{assets::Assets, player::Player, settings::GameSettings};

// * need this to stop the spot light being like an oval
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
async fn main() -> Result<(), macroquad::Error> {
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
    set_pc_assets_folder("assets");
    let assets = Assets::new().await?;

    let material = load_material(
        ShaderSource::Glsl {
            vertex: shadowgame::assets::SHADOWBLOB_SHADER_VERT,
            fragment: shadowgame::assets::SHADOWBLOB_SHADER_FRAG,
        },
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
    let mut player: Player = Player::new(
        vec2(0.0, 0.3),
        vec2(0.2, 0.2),
        vec2(0.5, -0.5),
        assets.player_idle,
        assets.player_walk,
    );
    let ground = Rect::new(-1.0, 0.4, 2.0, 0.2);
    let mut lights: Vec<Light> = vec![
        Light {
            pos_rad: vec3(0.5, 0.25, 0.2),
            color: vec3(0.81, 0.77, 0.67),
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

    let mut time = get_time();

    let settings: GameSettings = Default::default();

    loop {
        set_camera(&camera);

        clear_background(DARKGRAY);

        draw_rectangle(ground.x, ground.y, ground.w, ground.h, WHITE);
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

        let delta = get_time() - time;
        let mut move_dir = Vec2::ZERO;
        if is_key_down(KeyCode::A) {
            move_dir.x -= settings.player_move_speed_x * delta as f32;
        }

        if is_key_down(KeyCode::D) {
            move_dir.x += settings.player_move_speed_x * delta as f32;
        }

        player.set_vel(move_dir);

        camera.target.x += move_dir.x;

        player.update();

        time = get_time();
        next_frame().await
    }
}
