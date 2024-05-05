#[repr(C)]
#[derive(Clone, Debug)]
pub struct ImplPartiallyFuncAndFields {
    pub one: u8,
    pub two: i8,
    pub tree: f32,
    pub four: bool,
    pub five: String,
}


#[no_mangle]
pub fn say(_self: &ImplPartiallyFuncAndFields, word: &str) {
    println!("> shared_2");
    println!("say: {}", word);
}
