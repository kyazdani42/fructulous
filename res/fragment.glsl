#version 440 core
in vec4 gl_FragCoord;

out vec4 FragColor;

uniform int maxIter;
uniform float zoom;
uniform float xOffset;
uniform float yOffset;

void main()
{
    double x = double(gl_FragCoord.x);
    double y = double(gl_FragCoord.y);
    double cRe = (x / 200.0 - 4.0) / double(zoom) + double(xOffset);
    double cIm = (y / 200.0 - 4.0) / double(zoom) + double(yOffset);
    double zRe = 0.0;
    double zIm = 0.0;
    double tmp = 0.0;

    int i = -1;

    while (++i < maxIter && (zRe*zRe+zIm*zIm) < 4.0) {
        tmp = zRe;
        zRe = zRe * zRe - zIm * zIm + cRe;
        zIm = 2.0 * tmp * zIm + cIm;
    }

    float r = 0.0;
    float g = 0.0;
    float b = 0.0;
    if (i < maxIter) {
        r = log(log(float(zRe*zRe+zIm*zIm)) / log(4.0));
    }

    FragColor = vec4(r, g, b, 1.0);
}
