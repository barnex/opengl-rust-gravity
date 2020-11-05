/*

*/
#version 450 core

layout (local_size_x = 16) in; // TODO !!!!!!!!!!!!!!!!!!

layout(binding = 0, rgba8ui) uniform uimage2D  rendered; // output added here

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
	uint i = gl_GlobalInvocationID.x;
	vec2 pos = p[i];

	ivec2 size = imageSize(rendered);
	// ivec2 xy = ivec2((pos * SCALE) - 0.5) * size);
	ivec2 xy = size / 2;
	imageAtomicAdd(rendered, xy, RGB(1, 1, 1));
}
