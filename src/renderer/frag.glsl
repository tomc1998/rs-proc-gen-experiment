#version 150 core

uniform sampler2D tex;

in vec4 v_col;
in vec2 v_uv;

out vec4 col;

void main() {
  // Don't render if we're totally transparent, this means we avoid the funny
  // depth buffer squares and we don't have to z-sort
  vec4 tinted_tex_col = v_col * texture(tex, v_uv);
  if (tinted_tex_col.a <= 0.0) {
    discard;
  }
  col = tinted_tex_col;
}
