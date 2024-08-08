#version 300 es
precision mediump float;

// uniform vec3 u_color;
// uniform float u_time;

in vec2 f_position;

out vec4 color;

void main() {
  // float r = abs(2 * fract(f_position.x) - 1.0);
  // float g = abs(2 * fract(f_position.y) - 1.0);
  // color = vec4(u_color.r, g, 1.0, 1.0);

  color = vec4(0.4f, 0.45f, 0.5f, 0.4f);
}