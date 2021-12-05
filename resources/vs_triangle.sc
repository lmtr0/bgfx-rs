$input a_position, a_color0

// vertex shader
// #include "../sys/bgfx/src/bgfx_shader.sh"


void main() {
    gl_Position = vec4(a_position, 1.0);
    // gl_Color = a_color0
}