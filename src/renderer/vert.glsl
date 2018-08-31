#version 150 core

uniform Transform {
    mat4 proj;
    mat4 view;
};

in vec3 pos;
in vec4 col;
in vec2 uv;

out vec4 v_col;
out vec3 v_pos;
out vec2 v_uv;

void main() {
    gl_Position = proj * view * vec4(pos, 1.0);
    v_col = col;
    v_uv = uv;
    v_pos = pos;
}
