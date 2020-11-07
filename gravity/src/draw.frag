/*

*/
#version 450 core

in  vec2 frag_tex_coord;
out vec4 output_color;

layout(binding = 3) uniform usampler2D photon; // photon map (see photon.glsl)

#define PHOTON_NORM (8.0)     

void main() {
	vec2 start = frag_tex_coord;
	vec3 ph = texture(photon, start).rgb * (0.1);
	output_color = vec4(ph, 1.0);
}