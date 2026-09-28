# tokendemo

An **end-to-end acceptance program** for BORUIX, verifying that the console token (console owner) truth can be read faithfully from user space.

[简体中文](README.md)

## What it tests

Each terminal node carries a **holder** record (who owns this console), readable through the standard file status query. This program reads that value through **public interfaces only**, touching no kernel internals.

## Three invariants it pins down

| # | Invariant |
| --- | --- |
| 1 | **The holder of your own standard streams is yourself** — when a process creates fd 0/1/2, the three nodes register that process as their holder |
| 2 | **"Is a terminal" and "has a holder" are true together** — both values come from **the same node**, retrieved in **one query**, so they cannot contradict each other |
| 3 | **The holder is a real PID, not a constant** — the program compares what it read against `getpid()`. If the kernel misreported the holder (filling in 0 or 1, say), the assertion fails |

## Honest boundary

The program **only reads** the holder; it does **not** test handover. Who should hold the token and when it moves are user-space policy, and the kernel offers no handover verb.

So what it proves is "the holder truth can be read faithfully from user space", **not** "token handover works".

## Usage

Run it standalone; no arguments.

```
[tokendemo] fd 0/1/2: owner=<pid> is_terminal=1 OK
[tokendemo] PASS
```

## Exit codes

| Exit code | Meaning |
| --- | --- |
| `0` | Everything passed |
| Non-zero | Number of the first failing check |

## Building

```bash
cargo build --release
```

## Layout

```
tokendemo/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the three invariant checks
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — provides the file status query interface
- [`consoled`](https://github.com/BRX-Boruix/consoled) — the console daemon

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
