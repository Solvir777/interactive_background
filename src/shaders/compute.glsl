#version 460

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;


layout(set = 0, binding = 0) writeonly uniform image2D img;
layout(push_constant) uniform PushConstants {
    float time;
} data;

float mod289(float x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 mod289(vec4 x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 perm(vec4 x){return mod289(((x * 34.0) + 1.0) * x);}

float noise(vec3 p){
    vec3 a = floor(p);
    vec3 d = p - a;
    d = d * d * (3.0 - 2.0 * d);

    vec4 b = a.xxyy + vec4(0.0, 1.0, 0.0, 1.0);
    vec4 k1 = perm(b.xyxy);
    vec4 k2 = perm(k1.xyxy + b.zzww);

    vec4 c = k2 + a.zzzz;
    vec4 k3 = perm(c);
    vec4 k4 = perm(c + 1.0);

    vec4 o1 = fract(k3 * (1.0 / 41.0));
    vec4 o2 = fract(k4 * (1.0 / 41.0));

    vec4 o3 = o2 * d.z + o1 * (1.0 - d.z);
    vec2 o4 = o3.yw * d.x + o3.xz * (1.0 - d.x);

    return o4.y * d.y + o4.x * (1.0 - d.y);
}

float v(vec2 posp, float wl, float ti){
    vec2 point = imageSize(img) * posp;
    vec2 pos = gl_GlobalInvocationID.xy;
    float a = noise(vec3(pos * 0.05, data.time * 0.25));
    float dist = length(pos - point) * 0.05 + a - data.time * ti;
    return 0.5 + 0.5 * sin(dist * wl);
}

void main() {
    vec3 col = vec3(
        v(vec2(0.3, 0.7), 0.3, 1.5),
        v(vec2(0.25, 0.4), 3.14, 3.14),
        v(vec2(0.1, 0.8), 0.5, 0.1)
    );

    vec4 to_write = vec4(col, 1.0);
    imageStore(img, ivec2(gl_GlobalInvocationID), to_write);
}