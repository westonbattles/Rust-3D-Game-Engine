#version 410 core
layout (location = 0) in vec3 aPos;
uniform float aspectRatio;
uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;


void main()
{   
    vec4 pos = (projection * view * model  * vec4(aPos, 1.0));
    gl_Position = vec4(pos.x / aspectRatio, pos.y, pos.z, pos.w);
}