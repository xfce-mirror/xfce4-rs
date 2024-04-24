[![License](https://img.shields.io/badge/License-LGPL%20v2.1-blue.svg)](https://gitlab.xfce.org/bindings/xfce4-rs/COPYING)

# xfce4-rs

Experimental bindings of Xfce libraries for the Rust programming language. There are no concrete plans to port Xfce projects to Rust, these bindings are only for experimentation purposes.

### Status

| Library | Support |
| ------ | ------ |
| exo | TODO |
| garcon | TODO |
| libxfce4panel | TODO |
| thunarx | TODO |
| libxfce4ui | TODO |
| libxfce4util | WIP |
| xfconf | WIP |

### How to use

Add to your project's `Cargo.toml`:

```
[dependencies]
xfconf = {path = "/path/to/xfce4-rs/xfconf"}
```

Use as:

```
fn main() {
    xfconf::init().expect("Failed to initialize xfconf");
    let channel = xfconf::Channel::get("xfwm4");
    let value = channel.get_string("/general/title_font", "not found");
    println!("title_font: {}", value);
    xfconf::shutdown();
}
```

In the future we might publish the bindings to crates.io.
