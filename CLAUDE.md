# CLAUDE.md – fs-init

## What is this?

FreeSynergy Init — minimal bootstrap binary.
Clones the official FreeSynergy Store repository to the local node so packages
are available offline without requiring a pre-installed `git` binary.

## Rules

- Language in files: **English** (comments, code, variable names)
- Language in chat: **German**
- OOP everywhere: traits over match blocks, types carry their own behavior
- No CHANGELOG.md
- After every feature: commit directly

## Quality Gates (before every commit)

```
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo test
```

Every lib.rs / main.rs must have:
```rust
#![deny(clippy::all, clippy::pedantic, warnings)]
```

## Architecture

- Single binary `fs-init` — clap CLI, gix-based clone
- `clone_store` — shallow-clones the store repo via gix (no system git required)
- `default_store_dir` — resolves `~/.local/share/fsn/store`

## Dependencies

- `gix =0.80` (blocking HTTP transport, worktree mutation)
- `clap =4` (derive feature)
