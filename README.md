# Fusarium

Named after the fungi that affects crops, namely corn and its kernels.

It is a Operating System kernel based on Philipp Oppermann's [BlogOS](https://os.phil-opp.com/).

I've added some conditional compilation in anticipation of trying too boot this on a Raspberry Pi Zero W.

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
