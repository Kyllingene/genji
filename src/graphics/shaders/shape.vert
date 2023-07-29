#version 150
in vec2 position;
in vec4 color;

out vec4 v_color;

uniform mat4 perspective;
uniform mat4 matrix;

void main() {
    v_color = color;
    gl_Position = perspective * matrix * vec4(position, 1.0, 1.0);
}
