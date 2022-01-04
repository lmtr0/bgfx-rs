all:
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_triangle.sc -o vs_triangle.gl --type v --platform linux -p 440
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_triangle.sc -o fs_triangle.gl --type f --platform linux -p 440

	# @../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_triangle.sc -o vs_triangle.gl --type v --platform windows -p dx12
	# @../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_triangle.sc -o fs_triangle.gl --type f --platform windows -p dx12

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_triangle.sc -o vs_triangle.vk --type v --platform linux -p spirv
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_triangle.sc -o fs_triangle.vk --type f --platform linux -p spirv

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_triangle.sc -o vs_triangle.mt --type v --platform osx -p metal
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_triangle.sc -o fs_triangle.mt --type f --platform osx -p metal

	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_triangle.sc -o vs_triangle.es --type v --platform linux -p 320_es
	@../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_triangle.sc -o fs_triangle.es --type f --platform linux -p 320_es

clean:
	rm -rf *.h
	rm -rf *.mt
	rm -rf *.vk
	rm -rf *.gl
	rm -rf *.es