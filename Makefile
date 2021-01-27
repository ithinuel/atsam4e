TARGET=thumbv7em-none-eabihf
BIN_DIR=target/${TARGET}/release/examples

all: usb_poll helloworld blinky bootloader usb_dfu

usb_poll helloworld blinky:
	cargo build --target ${TARGET} --example $@ --release

bootloader usb_dfu:
	cargo build --release --target ${TARGET} --example bootloader --features bootloader
	cargo build --release --target ${TARGET} --example usb_dfu --features application

%.bin: %
	@rust-objcopy -O binary ${BIN_DIR}/$< ${BIN_DIR}/$@
	@rust-size ${BIN_DIR}/$<
	@exa -l ${BIN_DIR}/$@

flash_%: %.bin
	bossac -e -w -v -p /dev/ttyACM0 -b "${BIN_DIR}/$<"
	#@sam-ba_ 4 /dev/ttyACM0 at91sam4e8-ek ./flash.tcl "${BIN_DIR}/$<"

clean:
	cargo clean
