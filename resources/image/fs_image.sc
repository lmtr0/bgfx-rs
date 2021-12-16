$input v_texcoord0

uniform sampler u_texture;
#include "../../sys/bgfx/src/bgfx_shader.sh"

void main() {
    gl_FragColor = texture(u_texture, v_texcoord0.xy);
}