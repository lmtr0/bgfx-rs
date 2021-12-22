$input a_position

void main()
{
	gl_Position = mul(u_modelViewProj, vec4(a_position, 1.0) );
}