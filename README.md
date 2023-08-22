# lb2d

`lb2d` means `LXD(snap) network bridges to docker`.

This tool register LXD(snap) network bridges to docker's iptables chain `DOCKER-USER`.

See: <https://documentation.ubuntu.com/lxd/en/latest/howto/network_bridge_firewalld/>

## How to install

### Arch Linux

#### Requirements

- `rustup`
- `cargo-make`

How to install above packages: `pacman -S rustup cargo-make`

#### Build & Install

1. `git clone https://github.com/sifyfy/lb2d`
2. `cd /path/to/lb2d`
3. `cargo make install-by-pkgbuild`

## How to use

lb2d is installed as systemd oneshot unit.

If you add a lxd bridge, you run `systemctl restart lb2d`.
