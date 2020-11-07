
#version 450 core

layout(binding = 0, rgba8ui) uniform uimage2D dst;

layout (local_size_x = 16, local_size_y = 16) in;

void main() {
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);
	uvec3 v = imageLoad(dst, xy).rgb;
	v = uvec3(1, 10, 100);
	imageStore(dst, xy, uvec4(v, 255));
}
/*

#version 450 core

layout (local_size_x = 1, local_size_y=1) in; // TODO !!!!!!!!!!!!!!!!!!

layout(binding = 0, rgba8ui) uniform uimage2D rendered; // output added here

layout(std430, binding=1) buffer pos{
	vec2 p[];
};

// Colors represented as int,
// because atomicAdd only takes ints.
#define RGB(r, g, b) (((r)<<0) | ((g)<<8) | ((b)<<16))
#define RED    (RGB(1, 0, 0))
#define GREEN  (RGB(0, 1, 0))
#define BLUE   (RGB(0, 0, 1))

#define SCALE (0.1)

void main() {
	//uint i = gl_GlobalInvocationID.x;
	//vec2 pos = p[i];

	//ivec2 size = imageSize(rendered);
	// ivec2 xy = ivec2((pos * SCALE) - 0.5) * size);
	//ivec2 xy = size;

	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);
	imageAtomicAdd(rendered, xy, RGB(1, 2, 3));
	//imageStore(rendered, xy, vec4(0.5, 1.0, 0.1, 1.0));
}
*/