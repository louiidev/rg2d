#version 330 core
out vec4 FragColor;
in vec2 TexCoord;

uniform sampler2D sprite;

void main()
{
    FragColor = texture(sprite, TexCoord);
}