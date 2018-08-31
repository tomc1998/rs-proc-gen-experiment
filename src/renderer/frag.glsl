#version 150 core

uniform sampler2D tex;

uniform Fog {
  vec4 fog_center;   // Fog will be in a circle around this
  vec4 fog_color;    // Color of the fog
  float fog_density;   // Radius clear of fog
};

in vec4 v_col;
in vec3 v_pos;
in vec2 v_uv;

out vec4 col;

void main() {

  // Don't render if we're totally transparent, this means we avoid the funny
  // depth buffer squares and we don't have to z-sort
  vec4 tinted_tex_col = v_col * texture(tex, v_uv);
  if (tinted_tex_col.a <= 0.0) {
    discard;
  }

  // Apply fog
  float fog_dis = length(v_pos - fog_center.xyz);
  float f = 1.0 / exp(fog_dis * fog_density);
  vec4 fogged_col = (1 - f) * fog_color + f * tinted_tex_col;

  col = fogged_col;
}
