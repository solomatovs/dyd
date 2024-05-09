#[repr(C)]
#[derive(Clone, Debug)]
pub struct SharedTepl {
    pub name: String,
}

#[no_mangle]
pub fn say(_self: &SharedTepl) {
    println!("{} say: hello", _self.name);
}
