#include "../sys/bgfx/src/bgfx_shader.sh"
uniform vec4 u_color;

void main()
{
	gl_FragColor = u_color;
}