#[repr(C)]
#[derive(Clone, Debug)]
pub struct JudePlugin {
    pub word: String,
    pub one: u8,
    pub two: f32,
    pub tree: bool,
}

#[no_mangle]
pub fn who_am_i(_self: &JudePlugin) {
    println!("my name is Jude");
}
