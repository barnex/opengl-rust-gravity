/*

*/
#version 450 core

in  vec2 frag_tex_coord;
out vec4 output_color;

layout(binding = 0) uniform usampler2D rendered;

void main() {
	vec4 c = texture(rendered, frag_tex_coord);
	output_color = vec4(c.rgb, 1.0);
}