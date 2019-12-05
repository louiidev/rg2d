#version 330 core
layout (location = 0) in vec2 vertex;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

void main()
{
    gl_Position = projection * model * view * vec4(vertex.xy, 0.0, 1.0);
}