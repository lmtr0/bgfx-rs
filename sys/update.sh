#bin/bash
rm -rf bgfx
rm -rf bx
rm -rf bimg
git clone --depth=1 https://github.com/bkaradzic/bgfx   bgfx
git clone --depth=1 https://github.com/bkaradzic/bx     bx
git clone --depth=1 https://github.com/bkaradzic/bimg   bimg

# The SIZE_C(x) macros casuse issues for bindgen so we remove them
sed -i 's/UINT8_C//' bgfx/include/bgfx/defines.h
sed -i 's/UINT16_C//' bgfx/include/bgfx/defines.h
sed -i 's/UINT32_C//' bgfx/include/bgfx/defines.h
sed -i 's/UINT64_C//' bgfx/include/bgfx/defines.h
