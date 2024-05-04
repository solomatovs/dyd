#![feature(trace_macros)]

use std::ffi::{OsStr, OsString};

pub use jude::jude;


jude! (
    #[derive(Clone, Debug)]
    pub struct MyStruct {
        pub fn fn_one(self, one: u8),
        pub fn fn_two(&self, one: u8),
        fn fn_tree(&mut self, one: u8),
        fn fn_four(one: u8),
        fn fn_five(one: &mut u8) -> Self,
        fn fn_six(&mut self, one: u8) {
            self.one = one;
        },
        one: u8 = 8,
        two: u8 = {
            let s = 88;
            let dd = s / 4;
            dd
        },
        tree: String = String::from("my variable"),
        four: String = "my variable".to_string(),
        // four: &'a u8 = &9,
    }
);

impl TryInto<MyStruct> for OsString {
    type Error = libloading::Error;
    
    fn try_into(self) -> Result<MyStruct, Self::Error> {
        MyStruct::load_from_lib(self)
    }
}

// impl TryInto<MyStruct> for &str {
//     type Error = libloading::Error;
    
//     fn try_into(self) -> Result<MyStruct, Self::Error> {
//         let res = OsStr::new(self);
//         let res = res.to_os_string();
//         MyStruct::new(res)
//     }
// }

// impl FromResidual for MyStruct {
//     fn from_residual(residual: <Self as std::ops::Try>::Residual) -> Self {
//         todo!()
//     }
// }

impl Into<MyStruct> for &str {
    fn into(self) -> MyStruct {
        let res = OsStr::new(self);
        let res = res.to_os_string();
        MyStruct::load_from_lib(res).unwrap()
    }
}

fn main() {
    // trace_macros!(true);

    let lib = MyStruct::load_from_lib(OsString::from("libmy.dymod"));

    println!("{:?}", lib);

}
