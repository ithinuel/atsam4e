TARGET=thumbv7em-none-eabihf
BIN_DIR=target/${TARGET}/release/examples

ifeq ("$(shell uname -s)","Darwin")
	TARGET_TTY=/dev/tty.usbmodem101
else
	TARGET_TTY=/dev/ttyACM0
endif

all: usb_poll helloworld blinky bootloader usb_dfu

usb_poll helloworld blinky:
	cargo build --target ${TARGET} --example $@ --release

bootloader usb_dfu:
	cargo build --color=always --release --target ${TARGET} --example bootloader --features bootloader
	#cargo build --release --target ${TARGET} --example usb_dfu --features application
	cargo build --color=always --release --target ${TARGET} --example usb_dfu --features debug_on_buffer

%.bin: %
	@rust-objcopy -O binary ${BIN_DIR}/$< ${BIN_DIR}/$@
	@rust-size ${BIN_DIR}/$<
	@exa -l ${BIN_DIR}/$@

flash_%: %.bin
	bossac -e -w -v -p $(TARGET_TTY) -b "${BIN_DIR}/$<"
	#@sam-ba_ 4 /dev/ttyACM0 at91sam4e8-ek ./flash.tcl "${BIN_DIR}/$<"

clean:
	cargo clean
