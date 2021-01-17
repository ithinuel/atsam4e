BIN_DIR=target/thumbv7em-none-eabihf/release/examples

all: usb_poll helloworld blinky

usb_poll helloworld blinky:
	cargo build --target thumbv7em-none-eabihf --example $@ --release

%.bin: %
	@arm-none-eabi-objcopy -O binary ${BIN_DIR}/$< ${BIN_DIR}/$@
	@exa -l ${BIN_DIR}/$@

flash_%: %.bin
	bossac -e -w -v -p /dev/tty.usbmodem101 -b "${BIN_DIR}/$<"
	#@sam-ba_ 4 /dev/ttyACM0 at91sam4e8-ek ./flash.tcl "${BIN_DIR}/$<"

clean:
	cargo clean
