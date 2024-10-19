use macroquad::prelude::*;
use miniquad::window::screen_size;

fn transform_shader_pos(v: &Vec3, ratio: f32) -> Vec3 {
    Vec3::new(v.x, v.y / ratio, v.z)
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
    let target = render_target(screen_width() as u32, screen_height() as u32);
    target.texture.set_filter(FilterMode::Nearest);

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

    let material = load_material(
        ShaderSource::Glsl {
            vertex: SHADOW_VERTEX_SHADER,
            fragment: SHADOW_FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: uniforms,
            ..Default::default()
        },
    )
    .unwrap();
    material.set_uniform("dims", screen_size());

    let shadow_cam: Camera2D = Camera2D {
        zoom: vec2(1.0, screen_width() / screen_height()),
        target: Vec2::new(0.0, 0.0),
        render_target: Some(target.clone()),
        ..Default::default()
    };

    let ratio = screen_width() / screen_height();

    let mut lights: Vec<Light> = vec![
        Light {
            pos_rad: Vec3::new(0.0, 0.0, 0.1),
            color: Vec3::new(0.3, 0.6, 0.7),
        },
        Light {
            pos_rad: Vec3::new(0.9, 0.2, 0.1),
            color: Vec3::new(0.5, 0.1, 0.1),
        },
        Light {
            pos_rad: Vec3::new(0.2, 0.8, 0.1),
            color: Vec3::new(0.1, 0.5, 0.1),
        },
        Light {
            pos_rad: Vec3::new(0.9, 0.9, 0.1),
            color: Vec3::new(0.1, 0.1, 0.5),
        },
    ];

    loop {
        set_camera(&shadow_cam);
        clear_background(WHITE);

        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(0., 0., 0.1, YELLOW);

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

        let player = &mut lights[0];
        if is_key_down(KeyCode::A) {
            player.pos_rad.x -= 0.01;
        }

        if is_key_down(KeyCode::D) {
            player.pos_rad.x += 0.01;
        }

        if is_key_down(KeyCode::W) {
            player.pos_rad.y += 0.01;
        }

        if is_key_down(KeyCode::S) {
            player.pos_rad.y -= 0.01;
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

#define MAX_LIGHTS 4
struct Light {
    vec3 pos_rad;
    vec3 color;
};
uniform Light lights[MAX_LIGHTS];

float in_circle(vec2 a, vec3 b) {
    vec2 ta = vec2(a.x - b.x, a.y - b.y);
    if (length(ta) <= b.z) {
        return length(ta);
    }
    return 0.0;
}

void main() {
    vec3 res = texture(Texture, uv).rgb;
    vec2 shader_pos = gl_FragCoord.xy / dims;
    shader_pos.y *= dims.y / dims.x;

    int found = 0;
    for (int i = 0; i < MAX_LIGHTS; i++) {
        Light light = lights[i];
        float dist = in_circle(shader_pos, light.pos_rad);
        if (dist > 0) {
            float intensity = (light.pos_rad.z - dist) / light.pos_rad.z;
            if (found == 0) {
                res = res * light.color * intensity;
                found = 1;
            } else {
                res = mix(res, light.color,  intensity);
            }
        }
    }
    if (found == 0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        gl_FragColor = vec4(res, 0.0);
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
