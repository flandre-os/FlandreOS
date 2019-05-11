# FlandreOS

Toy operating system for personal learning operating system related knowledge

## To install requirement tools:
```bash
rustup component add llvm-tools-preview
cargo install cargo-xbuild bootimage
sudo apt-fast install qemu qemu-kvm libvirt-bin bridge-utils virt-manager
sudo systemctl start libvirtd.service
sudo systemctl enable libvirtd.service
```

## To build:
```bash
cargo xbuild
```

## To run:
```bash
cargo xrun
```
