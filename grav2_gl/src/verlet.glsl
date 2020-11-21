/*
	Verlet (leapfrog) integration:
	update velocities and positions given accelartions.

	https://en.wikipedia.org/wiki/Leapfrog_integration
*/
#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (binding = 0, rg32f) uniform          image2D pos;
layout (binding = 1, rg32f) uniform          image2D vel;
layout (binding = 2, rg32f) uniform readonly image2D acc;

uniform float dt = 0.0001;

void main(){
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);

	vec2 p = imageLoad(pos, xy).xy;
	vec2 v = imageLoad(vel, xy).xy;
	vec2 a = imageLoad(acc, xy).xy;

	v = v + a * dt;
	p = p + v * dt;

	imageStore(pos, xy, vec4(p, 0.0, 0.0));
	imageStore(vel, xy, vec4(v, 0.0, 0.0));
}