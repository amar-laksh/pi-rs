build:
	arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -O2 -c ./kernel/boot.s -o ./kernel/boot.o
	xargo build --target arm-unknown-linux-gnueabihf --release
	arm-none-eabi-gcc -T kernel/linker.ld -o ./target/kernel.elf -ffreestanding -O2 -nostdlib kernel/boot.o target/arm-unknown-linux-gnueabihf/release/libpi_rs.rlib
	arm-none-eabi-objcopy ./target/kernel.elf -O binary ./target/kernel.img

run:
	make build
	qemu-system-arm -M raspi2 -kernel ./target/kernel.elf -serial stdio


clean:
	rm ./kernel/*.o
	rm -R ./target/
