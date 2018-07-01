#version 150 core

uniform sampler2D tex;

in vec4 v_col;
in vec2 v_uv;

out vec4 col;

void main() {
  col = v_col * texture(tex, v_uv);
}