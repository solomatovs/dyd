#![feature(trace_macros)]

pub use jude::jude;

jude! (
    #[derive(Clone, Debug)]
    pub struct ImplAllFuncAndFields {
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
    let lib = ImplAllFuncAndFields::default();

    println!("{:?}", lib);
}
