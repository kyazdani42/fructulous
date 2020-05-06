#version 330 core

in vec2 pos;

out vec4 FragColor;

uniform int maxIter;
uniform float zoom;
uniform float xOffset;
uniform float yOffset;

void main()
{
    float cRe = pos.x / zoom + xOffset;
    float cIm = pos.y / zoom + yOffset;
    float zRe = 0.0;
    float zIm = 0.0;

    float tmp;

    float r = 0.0;
    float g = 0.0;
    float b = 0.0;

    int i = -1;

    while (++i < maxIter && (zRe*zRe+zIm*zIm) < 4.0) {
        tmp = zRe;
        zRe = zRe * zRe - zIm * zIm + cRe;
        zIm = 2.0 * tmp * zIm + cIm;
    }

    if (i < maxIter) {
        r = (float(i) - log(log(zRe*zRe+zIm*zIm) / log(4.0))) / float(i);
    }

    FragColor = vec4(r, g, b, 1.0);
}
