## Rust for the DirectStorage SDK (unofficial)
This crate lets you call any [DirectStorage SDK](https://devblogs.microsoft.com/directx/directstorage-api-downloads/) API using code generated from the metadata describing the API. It is powered by the [windows](https://github.com/microsoft/windows-rs) crate.

Note: This is an unofficial experimental 🧪 crate.

### Generating metadata
```
cd .metadata
dotnet build
```

### Regenerating crate
```
cargo run -p tool_api
```

### Running DirectStorage sample
```
./crates/samples/dstorage/deploy.cmd
cargo run -p sample_dstorage
```