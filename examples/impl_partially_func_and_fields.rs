#![feature(trace_macros)]

use std::ffi::OsString;

pub use jude::jude;


jude! (
    #[derive(Clone, Debug)]
    pub struct ImplPartiallyFuncAndFields {
        pub fn fn_from_lib_1(self, one: u8),
        pub fn fn_from_lib_2(&self, one: u8),
        pub fn fn_from_lib_3(&mut self, one: u8),
        pub fn fn_from_lib_4(one: u8),
        pub fn fn_self_impl(&mut self, one: u8) {
            self.fiels_1 = one;
        },
        fiels_1: u8 = 8,
        field_2: u8 = {
            let s = 88;
            let dd = s / 4;
            dd
        },
    }
);

fn main() {
    let lib = ImplPartiallyFuncAndFields::load_from_lib(OsString::from("libmy.dymod"));

    println!("{:?}", lib);
}
