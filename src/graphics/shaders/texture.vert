#version 150
in vec2 position;
in vec4 color;
in vec2 tex_coords;

out vec4 v_color;
out vec2 v_tex_coords;

uniform mat4 perspective;
uniform mat4 matrix;

void main() {
    v_color = color;
    v_tex_coords = tex_coords;
    gl_Position = perspective * matrix * vec4(position, 1.0, 1.0);
}
