# Build and test this fork

The published Linux amd64 binary was compiled with Rust/Cargo 1.95.0. The
committed Cargo.lock and vendored patched mdns-sd select its exact Rust sources.
On a Linux amd64 build host, install Rust 1.95.0 and the platform prerequisites
(`build-essential`, `pkg-config`, `libssl-dev`, `iproute2`, `util-linux`, Python 3).

```sh
bash scripts/build.sh
```

The result is `target/release/bifrost` unless CARGO_TARGET_DIR is set. A later
rebuild may differ byte-for-byte due to toolchains, paths and system libraries.
The release checksum identifies the exact executable tested on Home Assistant.

## Corresponding-source archive

Download the corresponding-source asset beside the binary release and extract
it. It contains this tree, `vendor/` with locked registry dependencies, and
`.cargo/config.toml` selecting those local sources. With the toolchain and system
prerequisites already installed, build without fetching Rust dependencies:

```sh
CARGO_NET_OFFLINE=true bash scripts/build.sh
```

## Isolated tests

```sh
bash scripts/test.sh
```

Compilation runs normally. Test executables run through sudo/unshare in isolated
network namespaces containing loopback and a dummy documentation-address interface;
they do not connect to a real Zigbee2MQTT server or lights. The mDNS regression
suite refuses to run without the isolation marker. Read the script before running
it. The inherited refresh-count timing test can be flaky; see VALIDATION.md.

The Home Assistant packaging is maintained separately at
https://github.com/gmaclean1981/bifrost-hassio. It verifies the executable bundle's
SHA256 and builds on the pinned upstream container base with Debian updates.

Publication packaging and corresponding-source archive prepared October 29, 2026.

Publication packaging and corresponding-source archive prepared October 29, 2026.
