#version 100
precision highp float;

varying vec2 uv;

uniform vec2 start;
uniform vec2 size;
uniform int iterations;

vec3 color(float t) {
    t = clamp(t, 0.0, 1.0);

    return vec3(
        9.0 * (1.0 - t) * t * t * t,
        15.0 * (1.0 - t) * (1.0 - t) * t * t,
        8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t
    );
}

void main() {
    vec2 c = start + uv * size;
    vec2 z = vec2(0.0);

    int i;

    for (i = 0; i < iterations; i++) {
        z = vec2(
            z.x * z.x - z.y * z.y,
            2.0 * z.x * z.y
        ) + c;

        if (dot(z, z) > 4.0) {
            break;
        }
    }

    if (i == iterations) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    float magnitude = length(z);
    float smooth_i = float(i) + 1.0 - log(log(magnitude)) / log(2.0);
    float t = sqrt(smooth_i) * 0.05;

    gl_FragColor = vec4(color(t), 1.0);
}
