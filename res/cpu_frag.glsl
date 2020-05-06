#version 330 core
in vec3 pCol;

out vec4 pColor;

void main()
{
    pColor = vec4(pCol, 1.0);
}
