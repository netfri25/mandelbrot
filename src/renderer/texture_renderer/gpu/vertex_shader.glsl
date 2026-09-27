#version 100

attribute vec3 position;
attribute vec2 texcoord;

uniform mat4 Model;
uniform mat4 Projection;

varying vec2 uv;

void main() {
    uv = texcoord;
    gl_Position = Projection * Model * vec4(position, 1.0);
}
