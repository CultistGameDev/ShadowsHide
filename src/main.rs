use macroquad::prelude::*;

fn transform_shader_pos(v: &Vec3, ratio: f32) -> Vec3 {
    Vec3::new(v.x, v.y / ratio, v.z)
}

#[macroquad::main("macroquad game jam")]
async fn main() {
    let target = render_target(screen_width() as u32, screen_height() as u32);
    target.texture.set_filter(FilterMode::Nearest);

    let material = load_material(
        ShaderSource::Glsl {
            vertex: SHADOW_VERTEX_SHADER,
            fragment: SHADOW_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("pos_rad", UniformType::Float3),
                UniformDesc::new("dims", UniformType::Float2),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let shadow_cam: Camera2D = Camera2D {
        zoom: vec2(1.0, screen_width() / screen_height()),
        target: Vec2::new(0.0, 0.0),
        render_target: Some(target.clone()),
        ..Default::default()
    };

    let ratio = screen_width() / screen_height();
    let mut shadow_pos = Vec3::new(0.0, 0.0, 0.1);

    loop {
        set_camera(&shadow_cam);
        clear_background(RED);

        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(0., 0., 0.1, YELLOW);

        set_default_camera();
        clear_background(WHITE);

        gl_use_material(&material);
        material.set_uniform("pos_rad", transform_shader_pos(&shadow_pos, ratio));
        draw_texture_ex(
            &target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        draw_line(
            0.0,
            screen_height() / 2.0,
            screen_width(),
            screen_height() / 2.0,
            1.0,
            WHITE,
        );
        draw_line(
            screen_width() / 2.0,
            0.0,
            screen_width() / 2.0,
            screen_height(),
            1.0,
            WHITE,
        );
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        if is_key_down(KeyCode::A) {
            shadow_pos.x -= 0.01;
        }

        if is_key_down(KeyCode::D) {
            shadow_pos.x += 0.01;
        }

        if is_key_down(KeyCode::W) {
            shadow_pos.y += 0.01;
        }

        if is_key_down(KeyCode::S) {
            shadow_pos.y -= 0.01;
        }

        next_frame().await
    }
}

const SHADOW_FRAGMENT_SHADER: &'static str = r#"#version 330 core
in vec4 color;
in vec2 uv;

uniform sampler2D Texture;

uniform vec3 pos_rad;
uniform vec2 dims;

int in_circle(vec2 a, vec3 b) {
    vec2 ta = vec2(a.x - b.x, a.y - b.y);
    if (length(ta) <= b.z) {
        return 1;
    }
    return 0;
}

void main() {
    vec3 res = texture(Texture, uv).rgb;
    vec2 shader_pos = gl_FragCoord.xy / dims;
    shader_pos.y *= dims.y / dims.x;

    if (in_circle(shader_pos, pos_rad) == 1) {
        gl_FragColor = vec4(res, 1.0);
    } else {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
}"#;

const SHADOW_VERTEX_SHADER: &'static str = r#"#version 330 core
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

out vec2 uv;
out vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

uniform vec3 pos_rad;
uniform vec2 dims;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
"#;
