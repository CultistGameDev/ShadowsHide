#version 330 core

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

#define MIN_INTENSITY 0.06
#define BACKGROUND_INTENSITY 0.03

void main() {
  vec3 res = texture(Texture, uv).rgb;
  vec2 shader_pos = gl_FragCoord.xy / dims;
  shader_pos.y *= dims.y / dims.x;

  int found = 0;
  for (int i = 0; i < MAX_LIGHTS; i++) {
    Light light = lights[i];
    float dist = in_circle(shader_pos, light.pos_rad);
    if (dist > 0) {
      float intensity =
          max((light.pos_rad.z - dist) / light.pos_rad.z, MIN_INTENSITY);
      if (found == 0) {
        res = res * light.color * intensity;
        found = 1;
      } else {
        res = mix(res, light.color, intensity);
      }
    }
  }
  if (found == 0) {
    gl_FragColor = vec4(mix(res, lights[0].color.rgb, 0.1) * BACKGROUND_INTENSITY, 1.0);
  } else {
    gl_FragColor = vec4(res, 0.0);
  }
}
