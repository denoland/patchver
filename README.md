Tool to promote exisiting Deno binaries to a different version.

### Install

```
cargo install patchver
```

### Usage

Modify your code to use sui to get current version at run-time.

```rust
use once_cell::sync::Lazy;

static CHANNEL: Lazy<&str> = Lazy::new(|| {
    libsui::find_section("denover")
        .unwrap_or("stable")
});
```

Promote to new channel:

```
patchver deno deno_new --channel rc
```
