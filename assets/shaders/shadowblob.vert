#version 330 core
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
