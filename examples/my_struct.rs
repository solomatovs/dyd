#![feature(trace_macros)]

pub use jude::jude;

fn main() {
    trace_macros!(true);

    jude! (
        #[derive(Copy, Clone, Debug)]
        pub struct MyStruct {
            pub fn fn_one(self, one: u8),
            fn fn_two(&self, one: u8),
            fn fn_tree(&mut self, one: u8),
            fn fn_four(one: u8),
            fn fn_five(one: &mut u8) -> Self,
            fn fn_six(&self, one: &mut Self) {
                let member = 8;
                member;
            },
            one: u8 = 8,
            two: u8 = {
                let s = 88;
                let dd = s / 4;
                dd
            },
            // tree: String = String::from(""),
        }
    );
}
