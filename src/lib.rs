#![feature(trace_macros)]

use libloading::{Error, Library, Symbol};
use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};
use std::sync::Arc;


use std::sync::RwLock;
use std::fmt::Debug;
use std::sync::TryLockError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum JudeError {
    #[error("failed to take the lock {0}, try again later")]
    WouldBlock(String),

    #[error("poisoned lock {0}. please reload lib {0}")]
    PoisonedLock(String, String),

    // #[error("failed to load {0}: {1}")]
    // FailedToLoadLib(String, libloading::Error),

    // #[error("failed to load symbol {0} in lib {1}: {2}")]
    // FailedToLoadSymbol(String, String, libloading::Error),
    #[error("failed to load {0}")]
    FailedToLoadLib(libloading::Error),

    #[error("failed to load symbol {0}")]
    FailedToLoadSymbol(libloading::Error),
}

impl From<libloading::Error> for JudeError {
    fn from(item: libloading::Error) -> Self {
        JudeError::FailedToLoadLib(item)
    }
}

impl<T> From<TryLockError<T>> for JudeError {
    fn from(item: TryLockError<T>) -> Self {
        match item {
            TryLockError::WouldBlock => JudeError::WouldBlock("lib".into()),
            TryLockError::Poisoned(x) => JudeError::PoisonedLock(x.to_string(), "lib".into()),
        }
    }
}

#[macro_export]
macro_rules! as_item( ($i:item) => ($i) );

#[macro_export]
macro_rules! jude(
    (
        $(#[$attr:meta])*
        $vis:vis
        struct $name:ident $(<$($lifetime:lifetime),+>)* $body:tt
    ) => (
        jude!(parse
            [
                $(#[$attr])*
                $vis struct $name $(<$($lifetime),+>)*
            ]
            [impl $(<$($lifetime),+>)* $name $(<$($lifetime),+>)*]
            [$(<$($lifetime),+>)*]
            [] [] [] [] [] $body
        );
    );
    
    // на этом этапе все фуркции и поля были разбиты на блоки поэтому остался только {}
    // передаю все блоки output
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {}
    ) => (
        jude!(output $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
        );
    );

    // парсинг функции с реализацией
    // такие функции обладают телом функции: $body:block
    // их передаю в исходном виде и никак не изменяем
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident($($tt:tt)*) $(-> $ret:ty)? $body:block, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn)* $(#[$attr])* $vis $(<$($lifetime),+>)* fn $name($($tt)*) $(-> $ret)? $body ]
            { $($t)* }
        );
    );

    // парсинг функций без реализации
    // такие функции нужно разбить на части для корректного добавления в member_impl, field_impl, fn блоки
    // такиим функциям будет дописано тело функции, которое содержит вызов
    // из загруженной динамической библиотеки

    // это парсинг фукнции с &mut self первым аргументом
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(&mut self $(,)? $($item:ident:$ty:ty),*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name], 
                [self], [&mut Self,], [&mut self,], [self,], 
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }
        );
    );
    
    // это парсинг фукнции с &self первым аргументом
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(&self $(,$item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name],
                [self], [&Self,], [&self,], [self,],
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }
        );
    );

    // это парсинг фукнции с self первым аргументом
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(self $(,$item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name],
                [self], [Self,], [self,], [self,],
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }
        );
    );
    
    // это парсинг фукнции без первого аргумента с типами: self, &self, &mut self
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident($($item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name],
                [self], [], [&self,], [],
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }
        );
    );
    
    // после парсинга функций без реализации с первым аргументом:: self, &self, &mut self
    // происходит формирование блоков member_impl, field_impl, fn
    // - member_impl - список полей структуры, который будет отображен в блоке struct
    // - field_impl - это выражения через которые заполняются поля указателей через подгрузку динамической библиокети в конструкторе
    // - fn - список функций в struct_impl блоке где происходит вызов функции из динамической библиокеки
    // далее из этих блоков будет составлена struct и struct_impl блоки с полями и функциями
    // функции не имеющие реализацию будут иметь поле указатель (member_impl) с названием этой функции
    // данный указатель будет заполняться в конструкторе через загрузку динамической библиотеки
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            [$fnattr:tt],
            [fn $name:ident],
            [$self:tt],
            [$($member_impl_self:tt)*],
            [$($fn_struct_decl_self:tt)*],
            [$($fn_call_self:tt)*],
            [$($item:ident : $ty:ty),*],
            [$(-> $ret:ty)?],
            $($t:tt)*
        }
    ) => (
        jude!(parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* $name: fn($($member_impl_self)* $($ty),*) $(-> $ret)?, ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* $name, ]
            [ $($fn)* $fnattr fn $name($($fn_struct_decl_self)* $($item:$ty),*) $(-> $ret)? {
                ($self.$name)($($fn_call_self)* $($item),*)
            }]
            { $($t)* }
        );
    );
    
    // это парсинг полей структуры
    // значения которых вычисляется через выражение с блоком $body:block
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis
            $name:ident: $(<$($lifetime:lifetime),+>)* $typ:ty = $body:block, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* $(#[$attr])* $vis $name: $(<$($lifetime),+>)* $typ, ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* $name: $body, ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
            { $($t)* }
        );
    );

    // это парсинг полей структуры
    // значения которых вычисляется через явное назначение литерала
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis
            $name:ident: $(<$($lifetime:lifetime),+>)* $typ:ty = $body:literal, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* $(#[$attr])* $vis $name: $(<$($lifetime),+>)* $typ, ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* $name: $body, ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
            { $($t)* }
        );
    );

    // это парсинг полей структуры
    // значения которых вычисляется через явное назначение expr
    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis
            $name:ident: $(<$($lifetime:lifetime),+>)* $typ:ty = $($body:expr)+, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* $(#[$attr])* $vis $name: $(<$($lifetime),+>)* $typ, ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* $name: $($body)+, ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
            { $($t)* }
        );
    );

    (
        parse $struct_decl:tt $struct_impl:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis
            $name:ident: $(<$($lifetime:lifetime),+>)* $typ:ty, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_decl $struct_impl $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* $(#[$attr])* $vis $name: $(<$($lifetime),+>)* $typ, ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn)* ]
            { $($t)* }
        );
    );

    (
        output [$($struct_decls:tt)*] [$($struct_impl:tt)*] [$(<$($struct_lifetime:lifetime),+>)*]
            [ $($member_impl:tt)* ]
            [ $($member_not_impl:tt)* ]
            [ $($field_impl:tt)*]
            [ $($field_not_impl:tt, )*]
            [ $($fn:tt)* ]
    )
    => (
        $crate::as_item!(
            $($struct_decls)* {
                file_path: std::ffi::OsString,
                lib: std::sync::Arc<libloading::Library>,
                lock: std::sync::Arc<std::sync::RwLock<()>>,
                $($member_impl)*
                $($member_not_impl)*
            }
        );

        $crate::as_item!(
            $($struct_impl)* {
                $($fn)*
            }
        );

        $crate::as_item!(
            $($struct_impl)* {
                fn new(file_path: std::ffi::OsString) -> Result<Self, $crate::JudeError> {
                    let lock = std::sync::RwLock::new(());
                    let lib = unsafe {
                        libloading::Library::new(&file_path)
                    }?;

                    let res = Self {
                        $($field_impl)*
                        $(
                            $field_not_impl: {
                                let symbol = unsafe {
                                    lib.get(stringify!($field_not_impl).as_bytes())
                                }?;
                                
                                *symbol
                            },
                        )*
                        file_path,
                        lib: std::sync::Arc::new(lib),
                        lock: std::sync::Arc::new(lock),
                    };

                    Ok(res)
                }
            }
        );
    );
);
