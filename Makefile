.PHONY: build
build:
	cargo build --target x86_64-unknown-uefi
	mkdir -p esp/efi/boot
	cp target/x86_64-unknown-uefi/debug/os.efi esp/efi/boot/bootx64.efi

.PHONY: run
run:
	make build
	qemu-system-x86_64 -enable-kvm \
	-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
	-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
	-drive format=raw,file=fat:rw:esp

.PHONY: clean
clean:
	cargo clean
	rm -rf esp
