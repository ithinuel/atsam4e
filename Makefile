BIN_DIR=target/thumbv7em-none-eabihf/release/examples

one: helloworld

helloworld blinky:
	cargo build --target thumbv7em-none-eabihf --example $@ --release
	@arm-none-eabi-objcopy -O binary ${BIN_DIR}/$@ ${BIN_DIR}/$@.bin
	@exa -l ${BIN_DIR}/$@.bin
	@sam-ba_64 /dev/ttyACM0 at91sam4e8-ek ./flash.tcl "${BIN_DIR}/$@.bin"
