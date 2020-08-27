send_file {Flash} [lindex $argv 3] 0x400000 0
#send_file {Flash} "target/thumbv7em-none-eabihf/release/examples/blinky.bin" 0x400000 0
FLASH::ScriptGPNMV 2

