/*

*/
#version 450 core

in  vec2 frag_tex_coord;
out vec4 output_color;

layout(binding = 0) uniform usampler2D rendered;

void main() {
	output_color = texture(rendered, frag_tex_coord);
}