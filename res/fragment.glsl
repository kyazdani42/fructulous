#version 440 core
in vec4 gl_FragCoord;

out vec4 FragColor;

uniform int maxIter;
uniform float zoom;
uniform float xOffset;
uniform float yOffset;
uniform float algType;
uniform int colorType;
uniform float time;

void main()
{
    float B = 256.0;

    double x = double(gl_FragCoord.x);
    double y = double(gl_FragCoord.y);
    double cRe = (x / 200.0 - 4.0) / double(zoom) + double(xOffset);
    double cIm = (y / 200.0 - 2.5) / double(zoom) + double(yOffset);
    double zRe = 0.0;
    double zIm = 0.0;
    double tmp = 0.0;

    int i = -1;

    // Mandelbrot
    if (algType == 1.0 || algType == 2.0) {
        // classique
        double mult = 2.0;

        // TriFractal
        if (algType == 2.0) {
            mult = -2.0;
        }

        while (++i < maxIter && zRe*zRe+zIm*zIm < B*B) {
            tmp = zRe;
            zRe = zRe * zRe - zIm * zIm + cRe;
            zIm = mult * tmp * zIm + cIm;
        }
    // Julia 2 sides
    } else if (algType == 3.0) {
        float n = 3.0;
        while (++i < maxIter && zRe*zRe+zIm*zIm < B*B) {
            tmp = double(pow(float(zRe*zRe+zIm*zIm), n / 2.0)) * cos(n * atan(float(zIm), float(zRe))) + cRe;
            zIm = double(pow(float(zRe*zRe+zIm*zIm), n / 2.0)) * sin(n * atan(float(zIm), float(zRe))) + cIm;
            zRe = tmp;
        }
    }

    if (i >= maxIter) {
        FragColor = vec4(0.0,0.0,0.0,1.0);
        return;
    }

    vec3 col = vec3(0.0);

    float sl = i - log2(log2(float(zRe*zRe+zIm*zIm))) + 4.0;
    float al = smoothstep(-0.1, 0.0, sin(0.5*6.2831));
    tmp = mix( i, sl, al );

    vec3 c;
    float wave = sin(time);
    if (colorType == 1) {
        c = vec3(0.0,0.6,wave);
    } else if (colorType == 2) {
        c = vec3(wave,0.6,0.0);
    } else if (colorType == 3) {
        c = vec3(wave,0.0,0.6);
    } else if (colorType == 4) {
        c = vec3(0.6,0.0,wave);
    } else if (colorType == 5) {
        c = vec3(0.6,wave,0.0);
    } else {
        c = vec3(0.0,wave,0.6);
    }

    col += 0.5 + 0.5 * cos(3.0 + float(tmp)*0.15 + c);
    FragColor = vec4(col, 1.0);
}
