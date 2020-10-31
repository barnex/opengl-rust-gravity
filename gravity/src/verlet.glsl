/*
	Verlet (leapfrog) integration:
	update velocities and positions given accelartions.

	https://en.wikipedia.org/wiki/Leapfrog_integration
*/
#version 450 core

layout (local_size_x=16) in; // TODO !!!!!!!!!!!!!!!

layout(std430, binding=0) buffer pos{
	vec2 p[];
};

layout(std430, binding=1) buffer vel{
	vec2 v[];
};

layout(std430, binding=2) buffer acc{
	vec2 a[];
};


//uniform float dt;

void main(){
	uint i = gl_GlobalInvocationID.x;
	v[i] = p[i] * 2.0;
	//v = v + a * dt;
	//p = p + v * dt;

}