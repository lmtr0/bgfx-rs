all:
	@echo "Shaders Header"
	# ../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_image.sc -o vs_image.gl.h --type v --platform linux -p 440 --bin2c vs_image_gl
	# ../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_image.sc -o fs_image.gl.h --type f --platform linux -p 440 --bin2c fs_image_gl

	# ../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_image.sc -o vs_image.vk.h --type v --platform linux -p spirv --bin2c vs_image_vk
	# ../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_image.sc -o fs_image.vk.h --type f --platform linux -p spirv --bin2c fs_image_vk

	# ../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_image.sc -o vs_image.mt.h --type v --platform linux -p metal --bin2c vs_image_mt
	# ../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_image.sc -o fs_image.mt.h --type f --platform linux -p metal --bin2c fs_image_mt

	# ../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_image.sc -o vs_image.es.h --type v --platform linux -p 320_es --bin2c vs_image_es
	# ../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_image.sc -o fs_image.es.h --type f --platform linux -p 320_es --bin2c fs_image_es

	@echo "Shaders"

	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_image.sc -o vs_image.gl --type v --platform linux -p 440
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_image.sc -o fs_image.gl --type f --platform linux -p 440

	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_image.sc -o vs_image.vk --type v --platform linux -p spirv
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_image.sc -o fs_image.vk --type f --platform linux -p spirv

	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_image.sc -o vs_image.mt --type v --platform linux -p metal
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_image.sc -o fs_image.mt --type f --platform linux -p metal

	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f vs_image.sc -o vs_image.es --type v --platform linux -p 320_es
	../../sys/bgfx/.build/linux64_gcc/bin/shadercRelease -f fs_image.sc -o fs_image.es --type f --platform linux -p 320_es

clean:
	rm -rf *.h
	rm -rf *.mt
	rm -rf *.vk
	rm -rf *.gl
	rm -rf *.es