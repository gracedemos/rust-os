all:
	i686-elf-as boot/boot.s -o boot.o
	clang vga/vga.c --target=i686-elf -c -ffreestanding
	cd kernel; cargo build
	i686-elf-ld boot.o vga.o kernel/target/i686-elf/debug/libkernel.a -o rust-os -T linker.ld -nostdlib -z noexecstack

clean:
	rm *.o
	rm -rf kernel/target
