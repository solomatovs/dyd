#![feature(trace_macros)]
#![feature(maybe_uninit_as_bytes)]
#![feature(strict_provenance)]
#![feature(maybe_uninit_uninit_array)]


use std::path::PathBuf;
use std::{ffi::OsString, thread, time::Duration};
use std::{mem, ptr};
use std::mem::MaybeUninit;
// use std::mem;
use std::mem::offset_of;
use std::mem::size_of;
use std::ptr::addr_of;
use std::ptr::addr_of_mut;
use std::io::Write;
use std::ffi::c_void;

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

// macro_rules! offset_of {
//     ($ty:ty, $field:ident) => {
//         unsafe { &(*(0 as *const $ty)).$field as *const _ as usize }
//     }
// }

trait StructSlice: Sized {
    fn as_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }

    fn as_struct(bytes: &[u8]) -> &Self {
        unsafe { &*(bytes.as_ptr() as *const Self) }
    }
}

fn main() -> Result<(), libloading::Error> {
    #[repr(C)]
    #[derive(Debug)]
    struct JudePluginInner {
        // pub config_path: PathBuf,
        pub one: u8,
        pub two: f32,
    }
    #[repr(C)]
    #[derive(Debug)]
    pub struct JudePlugin {
        // pub config_path: PathBuf,
        pub one: u8,
        pub two: f32,
        who_am_i: fn(&Self),
        __from_file: std::ffi::OsString,
        __from_lib: std::sync::Arc<libloading::Library>,
        __modified: std::time::SystemTime,
    }
    impl StructSlice for JudePlugin {}
    impl JudePlugin {}
    impl JudePlugin {
        pub fn who_am_i(&self) {
            (self.who_am_i)(self)
        }
        pub fn new(
            lib_path: std::ffi::OsString,
            config_path: PathBuf,
        ) -> Result<Self, libloading::Error> {
            let sizeof = std::mem::size_of::<JudePlugin>();
            
            let lib = std::sync::Arc::new(unsafe {
                libloading::Library::new(&lib_path)
            }?);
            // let symbol: libloading::Symbol<fn(std::ffi::OsString, PathBuf) -> Result<*const [u8], libloading::Error>> = unsafe {
            //     lib.get("new".as_bytes())
            // }?;
            // let mut res =  symbol(lib_path.clone(), config_path)?;

            let symbol: libloading::Symbol<fn(std::ffi::OsString, PathBuf) -> u8> = unsafe {
                lib.get("new".as_bytes())
            }?;
            let mut res =  symbol(lib_path.clone(), config_path);
            // let res = ptr::from_ref(&res);
            // println!("{:#?}", res);
            let mut src_ptr = ptr::addr_of_mut!(res) as *const u8;
            println!("base_ptr: {:#?}", src_ptr);
            let off = offset_of!(Self, two);
            let size = off + mem::size_of::<f32>();
            // Промежуточный массив байт
            let mut buffer: Vec<MaybeUninit<u8>> = Vec::with_capacity(size);
            buffer.set_len(size);


            let modified = std::fs::metadata(&lib_path).unwrap();
            let modified = modified.modified().unwrap();

            let mut res_2: MaybeUninit<JudePlugin> = MaybeUninit::uninit();

            let dst_ptr = res_2.as_ptr();

            let p = res_2.as_mut_ptr();
            unsafe {
                p.copy_from(src_ptr, size);
                let off = offset_of!(Self, one);
                let off = offset_of!(Self, two);

                let off = offset_of!(Self, two);
                let ptr = *(src_ptr.byte_add(off).cast());
                addr_of_mut!((*p).two).swap(ptr);
                
                // let off = offset_of!(Self, config_path);
                // let ptr = *(base_ptr.byte_add(off).cast());
                // addr_of_mut!((*p).config_path).swap(ptr);

                let sym = lib.get("who_am_i".as_bytes())?;
                addr_of_mut!((*p).who_am_i).write(*sym);

                addr_of_mut!((*p).__from_file).write(lib_path);
                addr_of_mut!((*p).__from_lib).write(lib);
                addr_of_mut!((*p).__modified).write(modified);
            };

            


            
            // unsafe {};
            // let ptr = res_2.as_mut_ptr();
            // let offset = offset_of!(JudePlugin, one);
            // let addr_source = unsafe { base_ptr.byte_add(offset) };
            // let addr_target = unsafe {addr_of_mut!((*ptr).one).write(val)};
            // unsafe {addr_target.with_addr(addr_source)};
            let res = unsafe {res_2.assume_init()};
            // todo!();

            // let res = Self {
            //     who_am_i: {
            //         let symbol = unsafe { lib.get("who_am_i".as_bytes()) }?;
            //         *symbol
            //     },
            //     one: unsafe {
            //         let ptr = base_ptr.byte_add(offset_of!(Self, one));
            //         let ptr = ptr.cast();
            //         *ptr
            //     },
            //     two: unsafe {
            //         let ptr = base_ptr.byte_add(offset_of!(Self, two));
            //         let ptr = ptr.cast();
            //         *ptr
            //     },
            //     config: unsafe {
            //         let ptr = base_ptr.byte_add(offset_of!(Self, two));
            //         let ptr = ptr.cast();
            //         *ptr
            //     },
            //     __from_file: lib_path,
            //     __from_lib: lib,
            //     __modified: modified,
            // };

            let base_ptr_res = ptr::addr_of!(res);


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