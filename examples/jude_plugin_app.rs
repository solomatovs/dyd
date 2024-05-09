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
