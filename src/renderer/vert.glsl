#version 150 core

in vec3 pos;
in vec4 col;
in vec2 uv;

uniform Transform {
    mat4 u_proj;
    mat4 u_view;
};

out vec4 v_col;
out vec2 v_uv;

void main() {
    gl_Position = vec4(pos, 1.0) * u_proj;
    v_col = col;
    v_uv = uv;
}
