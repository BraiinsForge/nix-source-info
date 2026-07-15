# nix-source-info

`nix-source-info` makes a Nix flake's source metadata available to a Rust
program at runtime. It uses
[`ver-stub`](https://github.com/cbeck88/ver-stub-rs) to inject the flake's
`sourceInfo` JSON into already-built binaries, so changing version metadata does
not require recompiling them.

The repository provides:

- the `nix-source-info` Rust crate, whose `SourceInfo::get()` method exposes the
  revision, dirty state, timestamp, NAR hash, store path, and related metadata;
- the Nix flake helper `lib.patchSourceInfo`, which copies a package and patches
  every executable in its `bin` directory with that metadata.

## Usage

Read the metadata from Rust:

```rust
use nix_source_info::SourceInfo;

if let Some(info) = SourceInfo::get() {
    println!("{} ({})", info.rev8(), info.timestamp());
}
```

Then wrap the program's Nix package with the helper, passing the source flake's
`sourceInfo`:

```nix
source-info.lib.patchSourceInfo pkgs self.sourceInfo my-package
```

`SourceInfo::get()` returns `None` when the binary has not been patched.
