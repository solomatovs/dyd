use std::ffi::OsString;

use jude::jude;

jude! (
    #[repr(C)]
    #[derive(Debug)]
    pub struct ImplPartiallyFuncAndFields {
        pub one: u8 = 1,
        pub two: i8 = -2,
        pub tree: f32 = 3.0,
        pub four: bool = true,
        pub five: String = String::from("two"),

        pub fn say(&self, word: &str),
    }
);

impl std::ops::Drop for ImplPartiallyFuncAndFields {
    fn drop(&mut self) {
        println!("self drop {:#?}", self);
    }
}

fn main() -> Result<(), libloading::Error> {
    let lib = ImplPartiallyFuncAndFields::load_from_lib(
        OsString::from("target/debug/examples/libshared_1.dylib")
    )?;

    lib.say("hello");

    let lib = ImplPartiallyFuncAndFields::load_from_lib(
        OsString::from("target/debug/examples/libshared_2.dylib")
    )?;

    lib.say("world");

    Ok(())
}
