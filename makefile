



target/arduino_nina.bin: target/thumbv6m-none-eabi/debug/arduino_nina
	/home/brandon/.arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/arduino_nina target/arduino_nina.bin



target/thumbv6m-none-eabi/debug/arduino_nina: FORCE
	cargo build



test:
	cargo test --target=x86_64-unknown-linux-gnu

FORCE:




