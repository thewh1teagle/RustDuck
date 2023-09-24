# RustDuck

Cross Platform duckdns.org updater

## Build
```ts
cargo build --release
```

## Usage
Get your token and domains from [duckdns](https://duckdns.org)
create `duckdns.config.json` file at home `~/` with the following
```json
{
    "token": "XXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXX",
    "domains": ["domain", "second_domain"],
    "duration": "5m"
}
```
Then simply run rustduck.exe

## Start at boot
#### Windows
create shortcut to duckdns.exe in `shell:startup` folder


### TODO
- [ ] User friendly installer
- [ ] Create config from `CLI`
- [ ] `winget` package
- [ ] `apt` package


#### Contribution are welcome!