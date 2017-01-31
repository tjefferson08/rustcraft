#version 150

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D grass_tex_sampler;

void main() {
  color = texture(grass_tex_sampler, v_tex_coords);
}

