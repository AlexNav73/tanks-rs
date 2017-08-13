#version 150 core

in vec3 a_Pos;
in vec2 a_TexCoord;
out vec2 v_TexCoord;

layout (std140)
uniform Locals {
	mat4 u_Transform;
    mat4 u_View;
    mat4 u_Proj;
};

void main() {
    v_TexCoord = a_TexCoord;
    gl_Position = u_Proj * u_View * u_Transform * vec4(a_Pos, 1.0);
    gl_ClipDistance[0] = 1.0;
}
