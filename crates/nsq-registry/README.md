# nsq-ir

`nsq-ir` defines the current registry-path map for the NSQ intermediate representation and adjacent runtime databases. It centralizes where the workspace expects language, runtime, package, and translation registries to live.

## Responsibilities
- publish the IR version string
- compute registry paths relative to a chosen root
- provide a small helper for opportunistic file reads
- print the resolved registry map from the binary

## Library surface
- `NSQ_IR_VERSION`
- `NsqIrRegistryPaths`
- `registry_paths(root)`
- `read_if_present(path)`

## Command
```bash
cargo run -p nsq-ir --release -- [root]
```

## Workspace links
- Helps orient runtime-native registry placement
- Useful when auditing or rebuilding the NSQ runtime/registry topology
