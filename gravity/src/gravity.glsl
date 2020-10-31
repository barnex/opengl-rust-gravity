/*
*/
#version 450 core

layout (local_size_x=16) in; // TODO !!!!!!!!!!!!!!!

layout(std430, binding=0) buffer pos{
	vec4 p[];
};

layout(std430, binding=1) buffer acc{
	vec4 a[];
};

void main(){
	uint i = gl_GlobalInvocationID.x;
	vec3 p = p[i].xyz;
	float r = length(p);
	a[i] = vec4(-p / (r*r*r), 0.0);
}