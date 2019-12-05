#version 330 core
layout (location = 0) in vec2 vertex;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 model;
uniform mat4 projection;

out vec2 TexCoord;
void main()
{
    gl_Position = modal * projection * vec4(vertex.xy, 0, 1);
    TexCoord = aTexCoord;
}