$input v_color0, v_texcoord0

#include "../../sys/bgfx/src/bgfx_shader.sh"

uniform vec4 s_texColor;

void main()
{
	vec4 color = s_texColor;
	int index = int(v_texcoord0.w*4.0 + 0.5);
	float alpha = index < 1 ? color.z :
		index < 2 ? color.y :
		index < 3 ? color.x : color.w;
	gl_FragColor = vec4(v_color0.xyz, v_color0.a * alpha);
}