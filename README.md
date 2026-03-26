# fs-init

Bootstrap binary for FreeSynergy — clones the official Store repository
to the local node so packages are available offline without requiring a
pre-installed `git` binary.

## Build

```sh
cargo build --release
cargo test
```

## Usage

```sh
# Clone to default location (~/.local/share/fsn/store)
fs-init

# Custom store URL
fs-init --store-url https://github.com/MyOrg/MyStore.git

# Custom target directory
fs-init --target-dir /opt/fs/store

# Specific branch
fs-init --branch dev
```

## Architecture

- Single binary `fs-init` — clap CLI, gix-based clone
- Uses `gix` for cloning (no system `git` required)
- Default store: `https://github.com/FreeSynergy/Store.git`
- Default target: `~/.local/share/fsn/store`
