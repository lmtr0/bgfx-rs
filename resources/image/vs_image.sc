$input a_position, a_texcoord0
$ouput v_texcoord0

#include "../../sys/bgfx/src/bgfx_shader.sh"

void main() {
    gl_Position = vec4(a_position, 1.0, 1.0);
    v_texcoord0 = vec4(a_texcoord.xy, 1.0, 1.0);
}