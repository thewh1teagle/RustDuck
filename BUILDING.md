# Building

Linux:

```console
sudo apt install "libayatana-appindicator3*"
```

# Update packages

NodeJS
```console
bunx npm-check-updates -u -t newest
# add -t newest for beta
```

Rust
```console
cargo install cargo-edit
cargo upgrade
cargo upgrade --pinned -p <some-tauri-beta-package>
cargo update
```
