/*
	Verlet (leapfrog) integration:
	update velocities and positions given accelartions.

	https://en.wikipedia.org/wiki/Leapfrog_integration
*/
#version 450 core

layout (local_size_x=1) in; // TODO !!!!!!!!!!!!!!!

layout(std430, binding=0) buffer pos{
	vec3 el[];
};


//uniform float dt;

void main(){
	uint i = gl_GlobalInvocationID.x;

	el[i] = vec3(10.0, 20.0, 30.0);

	//float p = imageLoad(pos, xy).r;
	//float v = imageLoad(vel, xy).r;
	//float a = imageLoad(acc, xy).r;

	//v = v + a * dt;
	//p = p + v * dt;

	//imageStore(pos, xy, vec4(p, 0.0, 0.0, 0.0));
	//imageStore(vel, xy, vec4(v, 0.0, 0.0, 0.0));
}