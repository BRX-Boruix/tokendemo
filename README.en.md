# tokendemo

A BORUIX acceptance test verifying that console token truth is readable from user space.

[简体中文](README.md)

## What it tests

Every terminal node records its owner, readable through the file status interface. This program reads
it only through the public API and touches no kernel internals.

- The three standard nodes — stdin, stdout, stderr — must each be a terminal
- Each of the three must have an owner
- "Is a terminal" and "has an owner" come from the same node in the same query, so they cannot disagree

## Usage

No arguments. It checks fds 0/1/2 and prints each one's terminal flag and owner:

```
[tokendemo] my pid = 12
[tokendemo] fd0 is_terminal=1 console_owner=1
[tokendemo] fd1 is_terminal=1 console_owner=1
[tokendemo] fd2 is_terminal=1 console_owner=1
[tokendemo] PASS: stdin/stdout/stderr all owned by this pid, all terminals
```

Note: the owner is not necessarily this process's pid. The file descriptor table is inherited; a child
sees the owner its parent registered.

## Exit codes

- `0` — all checks passed
- non-zero — something failed; a pid-query failure is always `1`, and failed checks report their count (failing lines start with `FAIL`)

## Building

```bash
cargo build --release
```

## Repository layout

```
tokendemo/
├── Cargo.toml    # package manifest
├── build.rs      # injects the linker script
├── linker.ld     # user-space segment layout
└── src/
    └── main.rs   # per-fd checks and result printing
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — the file status interface
- [`consoled`](https://github.com/BRX-Boruix/consoled) — console daemon

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
