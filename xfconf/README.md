# xfconf

This crate is a safe API wrapper around libxfconf, a library that can
talk to Xfce's configuration system.

## How to use

Add to your project's `Cargo.toml`:

```toml
[dependencies]
xfconf = "0.1"
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
