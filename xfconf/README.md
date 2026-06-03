# xfconf

This crate is a safe API wrapper around libxfconf, a library that can
talk to Xfce's configuration system.

## How to use

Add to your project's `Cargo.toml`:

```toml
[dependencies]
xfconf = "0.1"
```

The default feature-set of the crate will enable APIs available through
4.20.x.  If you'd like to use APIs only available in newer versions of
libxfconf, see the `[features]` section of `Cargo.toml` for
version-based features you can enable.  For example, to enable new APIs
present through version 4.21.2, you would instead specify:

```toml
[dependencies]
xfconf = { version = "0.1", features = ["v4_21_2"] }
```

## Example code:

```rust,no_run
use xfconf::ChannelExtManual;

fn main() {
    xfconf::init().expect("Failed to initialize xfconf");
    let channel = xfconf::Channel::get("xfwm4");
    let value = channel.get_property::<String>("/general/title_font")
        .unwrap_or_else(|| "Comic Sans 11".to_owned());
    println!("title_font: {}", value);
    unsafe { xfconf::shutdown() };
}
```
