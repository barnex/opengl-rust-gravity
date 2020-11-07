/*

*/
#version 450 core

in  vec2 frag_tex_coord;
out vec4 output_color;

layout(binding = 3) uniform usampler2D photon; // photon map (see photon.glsl)

void main() {
	vec2 start = frag_tex_coord;
	uvec3 rgb = texture(photon, start).rgb;
	uint num_particles = rgb.r | rgb.g << 8 | rgb.b << 16;
	float density = float(num_particles);
	float gamma = sqrt(density) * 0.15;
	vec3 color = vec3(1.0, 0.5, 0.2);
	output_color = vec4(gamma * color, 1.0);
}