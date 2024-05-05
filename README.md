jude allows you to describe a structure in one in one struct block, without having to write an impl block

```rust
pub use jude::jude;

jude! (
    #[derive(Clone, Debug)]
    pub struct ImplAllFuncAndFields {
        pub fn fn_self_mut_ref(&mut self, one: u8) {
            self.field_1 = one;
        },
        fn fn_self_ref(&self, one: u8) {
            println!("one: {}", one);
        },
        fn fn_self(self) -> Self {
            Self {
                field_1: 0,
                ..self
            }
        },
        fn new() -> Self {
            Self {
                field_1: 0,
                field_2: 0,
            }
        },
        
        field_1: u8 = 8,
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
```
jude allows you to define the functionality of the structure without implementation
so the load_from_lib method will be available through which you can load a shared library in which the desired functionality is defined

