Jude this is a small macros for the libloading that makes it easier to write plugins
Jude depends libloading only

You can write the structure without implementation, and load the functionality from the plugin

Code example
-------

```rust
// examples/jude_plugin.rs
#[repr(C)]
#[derive(Clone, Debug)]
pub struct JudePlugin {
    pub word: String,
    pub one: u8,
    pub two: f32,
    pub tree: bool,
}

#[no_mangle]
pub fn who_am_i(_self: &JudePlugin) {
    println!("my name is Jude");
}
```

```rust
// examples/jude_plugin_app.rs
use std::{ffi::OsString, thread, time::Duration};

use jude::jude;

jude! (
    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct JudePlugin {
        pub word: String = String::from("example string"),
        pub one: u8 = 1,
        pub two: f32 = 2.0,
        pub tree: bool = true,

        pub fn who_am_i(&self),
    }
);

fn main() -> Result<(), libloading::Error> {
    let mut lib =
        JudePlugin::_load_from(OsString::from("target/debug/examples/libjude_plugin.dylib"))?;

    loop {
        if let Ok(true) = lib._is_changed() {
            if let Err(e) = lib._reload() {
                println!("{:?}", e);
                break;
            }
        }

        lib.who_am_i();

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
```

![](docs/SceneCapture.gif)

Usage
-----

```toml
# Cargo.toml
[dependencies]
jude = "0.1.*"

```

Example build
-------

```
1, cargo build --examples
2. cargo run --example jude_plugin_app
3. change who_am_i func examples/jude_plugin.rs 
4. cargo build --exmaples
```
