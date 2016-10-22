default: run

.PHONY: clean

build/multiboot_header.o: src/asm/multiboot_header.asm
	mkdir -p build
	nasm -f elf64 src/asm/multiboot_header.asm -o build/multiboot_header.o

build/long_mode_init.o: src/asm/long_mode_init.asm
	mkdir -p build
	nasm -f elf64 src/asm/long_mode_init.asm -o build/long_mode_init.o

build/boot.o: src/asm/boot.asm
	mkdir -p build
	nasm -f elf64 src/asm/boot.asm -o build/boot.o

build/kernel.bin: cargo build/multiboot_header.o build/long_mode_init.o build/boot.o src/asm/linker.ld
	~/opt/bin/x86_64-pc-elf-ld --gc-sections -n -o build/kernel.bin -T src/asm/linker.ld build/multiboot_header.o build/long_mode_init.o build/boot.o target/x86_64-unknown-blizzardos-gnu/release/libos.a

build/os.iso: build/kernel.bin src/asm/grub.cfg
	mkdir -p build/isofiles/boot/grub
	cp src/asm/grub.cfg build/isofiles/boot/grub
	cp build/kernel.bin build/isofiles/boot/
	~/opt/bin/grub-mkrescue -o build/os.iso build/isofiles

run: build/os.iso
	qemu-system-x86_64 -cdrom build/os.iso

build: build/os.iso

clean:
	cargo clean

cargo:
	xargo build --release --target x86_64-unknown-blizzardos-gnu
