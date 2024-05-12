#![feature(trace_macros)]

use std::path::PathBuf;
use std::{ffi::OsString, thread, time::Duration};
use std::ptr;

// use jude::jude;

// fn main() -> Result<(), libloading::Error> {
//     // trace_macros!(true);

//     jude! (
//         pub struct JudePlugin {
//             pub config_path: PathBuf,
//             pub one: u8,
//             pub fn who_am_i(&self),

//             pub fn new(lib_path: std::ffi::OsString, config_path: PathBuf) -> Result<Self, libloading::Error>,
//         }
//     );

//     let mut lib = JudePlugin::new(
//         OsString::from("target/debug/examples/libjude_plugin_new.dylib"),
//         PathBuf::from("config_path"),
//     )?;

//     loop {
//         if let Ok(true) = lib._is_changed() {
//             if let Err(e) = lib._reload() {
//                 println!("{:?}", e);
//                 break;
//             }
//         }

//         lib.who_am_i();

//         thread::sleep(Duration::from_secs(1));
//     }

//     Ok(())
// }


fn main() -> Result<(), libloading::Error> {
    #[derive(Debug)]
    pub struct JudePluginInner {
        // pub config_path: PathBuf,
        // pub one: u8,
    }
    #[derive(Debug)]
    pub struct JudePlugin {
        // pub config_path: PathBuf,
        // pub one: u8,
        who_am_i: fn(&Self),
        __from_file: std::ffi::OsString,
        __from_lib: std::sync::Arc<libloading::Library>,
        __modified: std::time::SystemTime,
    }
    impl JudePlugin {}
    impl JudePlugin {
        pub fn who_am_i(&self) {
            (self.who_am_i)(self)
        }
        pub fn new(
            lib_path: std::ffi::OsString,
            config_path: PathBuf,
        ) -> Result<Self, libloading::Error> {
            let lib = std::sync::Arc::new(unsafe { libloading::Library::new(&lib_path) }?);
            let symbol: libloading::Symbol<fn(std::ffi::OsString, PathBuf) -> JudePluginInner> = unsafe {
                lib.get("new".as_bytes())
            }?;
            let _res =  symbol(lib_path.clone(), config_path);
            let modified = std::fs::metadata(&lib_path).unwrap();
            let modified = modified.modified().unwrap();
            let res = Self {
                who_am_i: {
                    let symbol = unsafe { lib.get("who_am_i".as_bytes()) }?;
                    *symbol
                },
                // config_path: res.config_path,
                // one: res.one,
                __from_file: lib_path,
                __from_lib: lib,
                __modified: modified,
            };

            println!("{:#?}", res);

            Ok(res)
        }
        fn _reload(&mut self) -> Result<(), libloading::Error> {
            let lib = unsafe { libloading::Library::new(&self.__from_file) }?;
            self.who_am_i = {
                let symbol = unsafe { lib.get("who_am_i".as_bytes()) }?;
                *symbol
            };
            let modified = std::fs::metadata(&self.__from_file).unwrap();
            let modified = modified.modified().unwrap();
            self.__from_lib = std::sync::Arc::new(lib);
            self.__modified = modified;
            Ok(())
        }
        fn _is_changed(&self) -> Result<bool, std::io::Error> {
            let modified = std::fs::metadata(&self.__from_file)?;
            let modified = modified.modified()?;
            match modified.duration_since(self.__modified) {
                Ok(x) => Ok(!x.is_zero()),
                Err(_) => Ok(false),
            }
        }
    }
    let lib = JudePlugin::new(
        OsString::from("target/debug/examples/libjude_plugin_new.dylib"),
        PathBuf::from("config_path"),
    );

    if let Err(e) = lib {
        println!("{}", e);
        return Err(e);
    }

    let mut lib = lib.unwrap();

    loop {
        if let Ok(true) = lib._is_changed() {
            if let Err(e) = lib._reload() {
                {
                    println!("{0:?}", e);
                };
                break;
            }
        }
        lib.who_am_i();
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}