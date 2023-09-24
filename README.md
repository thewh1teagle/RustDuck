<kbd>
<img src="https://github.com/thewh1teagle/RustDuck/assets/61390950/d865acf9-6b61-4336-96e1-c5bb4a5c1611" width="350px" height="200px">
</kbd>    

# RustDuck


Cross Platform duckdns.org updater  


## Automatic Installer
Download `rustduck_init` from [releases](https://github.com/thewh1teagle/RustDuck/releases/)  
Start it and follow the instructions

## Manual Install
Navigate into [rustduck folder](https://github.com/thewh1teagle/RustDuck/tree/main/rustduck) and build it
```shell
cargo build --release
```
Create config file named `config.json`
```json
{
    "token": "aaaaaaaa-bbbb-cccc-dddd-cccccccccccc",
    "domains": ["domain", "second_domain"],
    "duration": "10m"
}
```
Then run it
```shell
./rustduck -c config.json
```

### TODO
- [X] User friendly installer
- [ ] `winget` package
- [ ] `apt` package


#### Contribution are welcome!
