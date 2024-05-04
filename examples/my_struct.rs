#![feature(macro_metavar_expr)]
#![feature(trace_macros)]


pub use dyd::dyd;

fn main() {
    // trace_macros!(true);

    dyd! (
        #[derive(Copy, Clone, Debug)]
        pub struct MyStruct {
            pub fn fn_one(self, one: u8),
            fn fn_two(&self, one: u8),
            fn fn_tree(&mut self, one: u8),
            fn fn_four(one: u8),
            fn fn_five(one: &mut u8) -> Self,
            one: u8,
        }
    );
}
