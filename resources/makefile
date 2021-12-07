SHADERC?=../sys/bgfx/.build/linux64_gcc/bin/shadercRelease

all:
	${SHADERC} -f vs_triangle.sc 	-p 440 		--type v	-o vs_triangle.opengl
	${SHADERC} -f fs_triangle.sc 	-p 440 		--type f	-o fs_triangle.opengl

	${SHADERC} -f vs_triangle.sc 	-p spirv 	--type v	-o vs_triangle.spirv
	${SHADERC} -f fs_triangle.sc 	-p spirv 	--type f	-o fs_triangle.spirv


	${SHADERC} -f vs_triangle.sc 	-p metal 	--type v	-o vs_triangle.metal
	${SHADERC} -f fs_triangle.sc 	-p metal 	--type f	-o fs_triangle.metal