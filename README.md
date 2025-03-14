# Fusarium

Named after the fungi that affects crops, namely corn and its kernels.

It is a Operating System kernel based on Philipp Oppermann's [BlogOS](https://os.phil-opp.com/).

A lot of things are named `armv6a`, when it should probably be `armv6k` or `arm11` to better reflect the Pi Zero W target.

## How to run

For `x86_64` in `qemu`:

```sh
cargo run --features x86_64 --target x86_64-fusarium.json
```

For `armv6a` in `qemu` (WIP, serial console and a shoddy VGA text buffer works):

```sh
cargo run --features armv6a --target armv6a-fusarium.json
```

### Testing

For `x86_64` in `qemu`:

```sh
cargo test --features x86_64 --target x86_64-fusarium.json
```

Testing is not currently supported on `arm`

### Bare metal

I tried to get this to boot off of a Raspberry Pi Zero W to no luck so far.

I have several ideas for why that is the case (namely I should roll my own UART driver in case pins are wrong in `arm-pl011-uart`). The `aarch64-cpu` crate could also be removed. I found `heapless`, `tock-registers`, and `embbeded-graphics/text` late into development but all very useful. I suspect my VGA Text implementation can be improved by using transitions instead of clearing the screen each-rewrite. However, I also need to deal with the static mutable issue.

If you were to try to boot it, you'd need: a Pi Zero W, SD card, a way to power the Pi, a UART serial adapter (I used a Tigard). You would add the following to the SD card formatted with FAT32 and named BOOT:

* `bcm2708-rpi-zero-w.dtb` (might be optional)
* `bootcode.bin`
* `fixup.dat`
* `start.elf`

And a `config.txt` file with the following:

```txt
arm_64bit=0
enable_uart=1
uart_2ndstage=1
kernel=kernel.bin
```

`arm_64bit=0` is probably unnecessary.

## Bibliography (for the code at least)

I try to include code comments sourcing anything I may have found helpful in an implementation, but I believe these are all of them, if not they may be cited inline with code comments referencing the repository name or a link.

```txt
[1] ARM Limited. ARM1176JZF-S Technical Reference Manual Revision: r0p7. Retrieved from <https://documentation-service.arm.com/static/5e8e294efd977155116a6ca3?token=>
[2] Broadcom. 2012. Broadcom BCM2835 ARM Peripherals. Retrieved March 1, 2025 from <https://datasheets.raspberrypi.com/bcm2835/bcm2835-peripherals.pdf>
[3] Joe FitzPatrick. Applied Physical Attacks with Tigard. SecuringHardware.com. Retrieved March 13, 2025 from <https://learn.securinghardware.com/courses/applied-physical-attacks-with-tigard/>
[4] Charles Edward Gagnon. 2024. carloskiki/rpi-devenv. Retrieved March 13, 2025 from <https://github.com/carloskiki/rpi-devenv>
[5] goldilocks. 2018. Answer to “Raspberry Pi Zero W is ARMv6 or ARMv7?” Raspberry Pi Stack Exchange. Retrieved March 13, 2025 from <https://raspberrypi.stackexchange.com/a/83379>
[6] Thanos Koutroubas. Raspberry Pi Hardware. thanoskoutr/armOS Wiki. Retrieved March 13, 2025 from <https://github.com/thanoskoutr/armOS/wiki/Raspberry-Pi-Hardware#pi-addresses>
[7] Philipp Oppermann. 2025. phil-opp/blog_os. Retrieved March 13, 2025 from <https://github.com/phil-opp/blog_os>
[8] Philipp Oppermann. Writing an OS in Rust. Writing an OS in Rust. Retrieved March 13, 2025 from <https://os.phil-opp.com/>
[9] Federico Ponzi. FedericoPonzi/bare-metal-space-invaders. Retrieved March 13, 2025 from <https://github.com/FedericoPonzi/bare-metal-space-invaders/tree/main>
[10] Rust on Embedded Devices Working Group. 2025. rust-embedded/rust-raspberrypi-OS-tutorials. Retrieved March 13, 2025 from <https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials>
[11] Pietro T. 2023. BRA1L0R/raspberrypi-1-rust. Retrieved March 13, 2025 from <https://github.com/BRA1L0R/raspberrypi-1-rust>
[12] David Welch. 2025. dwelch67/raspberrypi-zero. Retrieved March 13, 2025 from <https://github.com/dwelch67/raspberrypi-zero>
[13] 2025. embedded-graphics/embedded-text. Retrieved March 13, 2025 from <https://github.com/embedded-graphics/embedded-text>
[14] 2025. tigard-tools/tigard. Retrieved March 13, 2025 from <https://github.com/tigard-tools/tigard>
[15] 2025. embedded-graphics/embedded-graphics. Retrieved March 13, 2025 from <https://github.com/embedded-graphics/embedded-graphics>
[16] 2025. raspberrypi/firmware. Retrieved March 13, 2025 from <https://github.com/raspberrypi/firmware>
[17] 2025. tio/tio. Retrieved March 13, 2025 from <https://github.com/tio/tio>
[18] BCM2835 Interrupt Controller - Embedded Xinu. Embedded Xinu. Retrieved March 13, 2025 from <https://xinu.cs.mu.edu/index.php/BCM2835_Interrupt_Controller>
[19] config.txt. Raspberry Pi Documentation. Retrieved March 13, 2025 from <https://www.raspberrypi.com/documentation/computers/config_txt.html>
[20] Inline assembly. Rust By Example. Retrieved March 13, 2025 from <https://doc.rust-lang.org/rust-by-example/unsafe/asm.html>
[21] Disallow references to static mut. The Rust Edition Guide. Retrieved March 13, 2025 from <https://doc.rust-lang.org/nightly/edition-guide/introduction.html>
[22] Cargo Reference - The Cargo Book. Retrieved March 13, 2025 from <https://doc.rust-lang.org/cargo/reference/index.html>
[23] Interrupts. The Embedded Rust Book. Retrieved March 13, 2025 from <https://docs.rust-embedded.org/book/intro/index.html>
[24] Preface. The Embedonomicon. Retrieved March 13, 2025 from <https://docs.rust-embedded.org/embedonomicon/>
[25] Processors. Raspberry Pi Documentation. Retrieved March 13, 2025 from <https://www.raspberrypi.com/documentation/computers/processors.html>
[26] Raspberry Pi Bare Bones. OSDev Wiki. Retrieved March 13, 2025 from <https://wiki.osdev.org/Raspberry_Pi_Bare_Bones>
[27] Raspberry Pi Bare Bones Rust. OSDev Wiki. Retrieved March 13, 2025 from <https://wiki.osdev.org/Raspberry_Pi_Bare_Bones_Rust>
[28] Raspberry Pi boards (raspi0, raspi1ap, raspi2b, raspi3ap, raspi3b, raspi4b). QEMU documentation. Retrieved March 13, 2025 from <https://www.qemu.org/docs/master/system/arm/raspi.html>
[29] The Rust Programming Language. The Rust Programming Language. Retrieved March 13, 2025 from <https://doc.rust-lang.org/book/>
[30] UART. Raspberry Pi GPIO Pinout. Retrieved March 13, 2025 from <https://pinout.xyz/pinout/uart>
[31] ARM1176JZF-S Technical Reference Manual r0p7. Retrieved March 13, 2025 from <https://developer.arm.com/documentation/ddi0301/h>
[32] BCM2835 datasheet errata - eLinux.org. Embedded Linux Wiki. Retrieved March 13, 2025 from <https://elinux.org/BCM2835_datasheet_errata#p153>
[33] AArch64 Bare-Metal program in Rust - Blog - Löwenware. Retrieved March 13, 2025 from <https://lowenware.com/blog/aarch64-bare-metal-program-in-rust/>
```
