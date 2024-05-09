use std::{ffi::OsString, thread, time::Duration};

use jude::jude;

jude! (
    #[repr(C)]
    #[derive(Debug, Clone)]
    pub struct SharedTepl {
        pub word: String = String::from("hello world"),

        pub fn say(&self),
    }
);

fn main() -> Result<(), libloading::Error> {
    let mut lib =
        SharedTepl::_load_from(OsString::from("target/debug/examples/libshared_1.dylib"))?;

    loop {
        if let Ok(true) = lib._is_changed() {
            if let Err(e) = lib._reload() {
                println!("{:?}", e);
                break;
            }
        }

        lib.say();

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
