use std::path::PathBuf;

#[repr(C)]
// #[repr(transparent)]
#[derive(Clone, Debug)]
pub struct JudePlugin {
    pub one: u8,
    pub two: f32,
    // pub config_path: PathBuf,
}

#[no_mangle]
pub fn new(
    _lib_path: std::ffi::OsString,
    config_path: PathBuf,
// ) -> Result<JudePlugin, libloading::Error> {
) -> JudePlugin {
    println!("call fn new");

    let res = JudePlugin {
        one: 3,
        two: 10.0,
        // config_path: PathBuf::from("werwer"),
    };

    res

    // Ok(res)
}

#[no_mangle]
pub fn who_am_i(_self: &JudePlugin) {
    println!("i am Jude");
}
