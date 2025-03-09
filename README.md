# Fusarium

Named after the fungi that affects crops, namely corn and its kernels.

It is a Operating System kernel based on Philipp Oppermann's [BlogOS](https://os.phil-opp.com/).

## How to run

For `x86_64` in `qemu`:

```sh
cargo run --features x86_64 --target x86_64-fusarium.json
```

For `armv6a` in `qemu` (WIP):

```sh
cargo run --features armv6a --target armv6a-fusarium.json
```

### Testing

For `x86_64` in `qemu`:

```sh
cargo test --features x86_64 --target x86_64-fusarium.json
```

Testing is not currently supported on `arm`

## Credits

For help making `arm` work:

* [rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) from the Rust on Embedded Devices Working Group
* [rpi-devenv](https://github.com/carloskiki/rpi-devenv) from  Charles Edward Gagnon
* "ARM1176JZF-S™ Revision: r0p7 Technical Reference Manual" from ARM, can be found [online here](https://developer.arm.com/documentation/ddi0301/h?lang=en)
* [Clarification on ARM revision for the Raspberry Pi Zero W](https://raspberrypi.stackexchange.com/a/83379) from `goldilocks` on StackOverflow
* [Raspberry Pi Bare Bones Rust](https://wiki.osdev.org/Raspberry_Pi_Bare_Bones_Rust) from OSDev Wiki
* [Raspberry Pi's documentation](https://www.raspberrypi.com/documentation/computers/processors.html)
* [Raspberry Pi Hardware wiki](https://github.com/thanoskoutr/armOS/wiki/Raspberry-Pi-Hardware) from the [armOs](https://github.com/thanoskoutr/armOS) repo by Thanos Koutroubas
