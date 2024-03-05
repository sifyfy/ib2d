# ib2d

`ib2d` means `incus bridges to docker`.

This tool register incus network bridges to docker's iptables chain `DOCKER-USER`.

See: <https://linuxcontainers.org/incus/docs/main/howto/network_bridge_firewalld/#prevent-connectivity-issues-with-incus-and-docker>

## How to install

### Arch Linux

#### Requirements

- `rustup`
- `cargo-make`

How to install above packages: `pacman -S rustup cargo-make`

#### Build & Install

1. `git clone https://github.com/sifyfy/ib2d`
2. `cd /path/to/ib2d`
3. `cargo make install-by-pkgbuild`

## How to use

ib2d is installed as systemd oneshot unit.

If you add a incus bridge, you run `systemctl restart ib2d`.
