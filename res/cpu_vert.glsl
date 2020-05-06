#version 330 core
layout (location = 0) in vec2 pPos;
layout (location = 1) in vec3 color;

out vec3 pCol;

void main()
{
    gl_Position = vec4(pPos, 0.0, 1.0);
    pCol = color;
}
