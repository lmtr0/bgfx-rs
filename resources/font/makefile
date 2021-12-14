all:
	@echo "Shaders Header"
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.gl.h --type v --platform linux -p 440 --bin2c vs_font_gl
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.gl.h --type f --platform linux -p 440 --bin2c fs_font_gl

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.vk.h --type v --platform linux -p spirv --bin2c vs_font_vk
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.vk.h --type f --platform linux -p spirv --bin2c fs_font_vk

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.mt.h --type v --platform linux -p metal --bin2c vs_font_mt
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.mt.h --type f --platform linux -p metal --bin2c fs_font_mt

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.es.h --type v --platform linux -p 320_es --bin2c vs_font_es
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.es.h --type f --platform linux -p 320_es --bin2c fs_font_es

	@echo "Shaders"

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.gl --type v --platform linux -p 440
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.gl --type f --platform linux -p 440

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.vk --type v --platform linux -p spirv
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.vk --type f --platform linux -p spirv

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.mt --type v --platform linux -p metal
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.mt --type f --platform linux -p metal

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.es --type v --platform linux -p 320_es
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.es --type f --platform linux -p 320_es

clean:
	rm -rf *.h
	rm -rf *.mt
	rm -rf *.vk
	rm -rf *.gl
	rm -rf *.es