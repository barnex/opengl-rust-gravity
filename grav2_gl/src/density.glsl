/*

*/
#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout(binding = 0, rg32f) uniform image2D  pos; 
layout(binding = 1, rgba8ui) uniform uimage2D photons; // output added here

uniform float scale = 200.0;

// Colors represented as int,
// because atomicAdd only takes ints.
#define RGB(r, g, b) (((r)<<0) | ((g)<<8) | ((b)<<16))
#define RED    (RGB(2, 0, 0))
#define YELLOW (RGB(1, 1, 0))
#define GREEN  (RGB(0, 2, 0))
#define CYAN   (RGB(0, 1, 1))
#define BLUE   (RGB(0, 0, 2))
#define PURPLE (RGB(1, 0, 1))

#define WEIGHT (8)

void main() {
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);
	ivec2 size = imageSize(photons);

	vec2 p = imageLoad(pos, xy).xy;
	ivec2 pix = ivec2(p * scale + size / 2);
	imageAtomicAdd(photons, pix, WEIGHT);

}
