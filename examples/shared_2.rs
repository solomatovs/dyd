#[repr(C)]
#[derive(Clone, Debug)]
pub struct SharedTepl {
    pub word: String,
}

#[no_mangle]
pub fn say(_self: &SharedTepl) {
    println!("shared_2 say: {}", _self.word);
}
