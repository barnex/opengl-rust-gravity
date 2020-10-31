/*
*/
#version 450 core

layout (local_size_x=16) in; // TODO !!!!!!!!!!!!!!!

layout(std430, binding=0) buffer pos{
	vec2 p[];
};

layout(std430, binding=1) buffer acc{
	vec2 a[];
};

void main(){
	uint i = gl_GlobalInvocationID.x;
	vec2 p = p[i];
	float r = length(p);
	a[i] = -p / (r*r*r);
}