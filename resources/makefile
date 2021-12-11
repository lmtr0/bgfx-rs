all:
	../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_triangle.sc -o vs_triangle.gl --type v --platform linux -p 440
	../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_triangle.sc -o fs_triangle.gl --type f --platform linux -p 440

	../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_triangle.sc -o vs_triangle.vk --type v --platform linux -p spirv
	../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_triangle.sc -o fs_triangle.vk --type f --platform linux -p spirv