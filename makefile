build:
	arm-linux-gnueabihf-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c ./kernel/boot.s -o ./kernel/boot.o
	xargo build --target arm-unknown-linux-gnueabihf --release
	arm-linux-gnueabihf-gcc -T kernel/linker.ld -o ./target/pi_studio.elf -ffreestanding -O2 -nostdlib kernel/boot.o target/arm-unknown-linux-gnueabihf/release/libpi_rs.rlib

run:
	make build
	qemu-system-arm -M raspi2 -kernel ./target/pi_studio.elf -serial stdio


clean:
	rm ./kernel/*.o
	rm -R ./target/
