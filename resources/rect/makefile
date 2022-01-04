all:
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_rect.sc -o vs_rect.gl --type v --platform linux -p 440
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_rect.sc -o fs_rect.gl --type f --platform linux -p 440

	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_rect.sc -o vs_rect.vk --type v --platform linux -p spirv
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_rect.sc -o fs_rect.vk --type f --platform linux -p spirv

	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_rect.sc -o vs_rect.mt --type v --platform osx -p metal
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_rect.sc -o fs_rect.mt --type f --platform osx -p metal

	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_rect.sc -o vs_rect.es --type v --platform linux -p 320_es
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_rect.sc -o fs_rect.es --type f --platform linux -p 320_es

clean:
	rm -rf *.h
	rm -rf *.mt
	rm -rf *.vk
	rm -rf *.gl
	rm -rf *.es