all:
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.gl --type v --platform linux -p 440
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.gl --type f --platform linux -p 440

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.vk --type v --platform linux -p spirv
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.vk --type f --platform linux -p spirv

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.mt --type v --platform osx -p metal
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.mt --type f --platform osx -p metal

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_font.sc -o vs_font.es --type v --platform linux -p 320_es
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_font.sc -o fs_font.es --type f --platform linux -p 320_es

clean:
	rm -rf *.h
	rm -rf *.mt
	rm -rf *.vk
	rm -rf *.gl
	rm -rf *.es