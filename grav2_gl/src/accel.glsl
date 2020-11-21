/*
	Acceleration function for 2D linear waves + damping.

	Acceleration = -gradient(height) - damping * velocity

	This is in "natural" units, leading to unit wave speed.
	Actual wave speed can be controlled by the time step (verlet.glsl).

	https://en.wikipedia.org/wiki/Wave_equation
*/
#version 450 core

layout (local_size_x = 16, local_size_y = 16) in;

layout (binding = 0, rg32f) uniform readonly  image2D position;
layout (binding = 1, rg32f) uniform writeonly image2D acceleration;

uniform vec2 sun_pos = vec2(0.0, 0.0);

void main(){
	ivec2 xy = ivec2(gl_GlobalInvocationID.xy);

	vec2 p = imageLoad(position, xy).xy - sun_pos;
	float r = length(p);
	vec2 a = -p / (r*r*r);

	imageStore(acceleration, xy, vec4(a, 0.0, 0.0));
}