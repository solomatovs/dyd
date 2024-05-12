use std::path::PathBuf;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct JudePlugin {
    // pub config_path: PathBuf,
    // pub one: u8,
}

#[no_mangle]
pub fn new(
    _lib_path: std::ffi::OsString,
    config_path: PathBuf,
) -> Result<JudePlugin, libloading::Error> {
    println!("call fn new");

    Ok(JudePlugin {
        // config_path,
        // one: 1,
    })
}

#[no_mangle]
pub fn who_am_i(_self: &JudePlugin) {
    println!("i am Jude");
}
