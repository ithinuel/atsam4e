TARGET=thumbv7em-none-eabihf
BIN_DIR=target/${TARGET}/release/examples

all: usb_poll helloworld blinky

usb_dfu usb_poll helloworld blinky:
	#cargo build --target ${TARGET} --example $@ --release

%.bin: %
	@cargo objcopy --target ${TARGET} --example $< --release -- -O binary ${BIN_DIR}/$@
	@exa -l ${BIN_DIR}/$@

flash_%: %.bin
	bossac -e -w -v -p /dev/ttyACM0 -b "${BIN_DIR}/$<"
	#@sam-ba_ 4 /dev/ttyACM0 at91sam4e8-ek ./flash.tcl "${BIN_DIR}/$<"

clean:
	cargo clean
