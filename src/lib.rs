#![feature(trace_macros)]

// use libloading::{Error, Library, Symbol};
// use std::path::{Path, PathBuf};
// use std::ffi::{OsStr, OsString};
// use std::sync::Arc;


// use std::sync::RwLock;
// use std::fmt::Debug;
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
        $vis:vis $(<$($lifetime:lifetime),+>)*
        struct $name:ident $body:tt
    ) => (
        jude!(parse [$(#[$attr])* $vis $(<$($lifetime),+>)* struct $name] [impl $name] [] [] [] $body);
    );
    
    // на этом этапе все фуркции и поля были разбиты на блоки поэтому остался только {}
    // передаю все блоки output
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {}
    ) => (
        jude!(output $decl $impl
            [$($member)*]
            [$($member_construct)*]
            [$($fn)*]
        );
    );

    // парсинг функции с реализацией
    // такие функции обладают телом функции: $body:block
    // их передаю в исходном виде и никак не изменяем
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident($($tt:tt)*) $(-> $ret:ty)? $body:block, $($t:tt)*
        }
    ) => (
        jude!(parse $decl $impl
            [ $($member)* ]
            [ $($member_construct)* ]
            [ $($fn)* $(#[$attr])* $vis $(<$($lifetime),+>)* fn $name($($tt)*) $(-> $ret)? $body ]
            { $($t)* }
        );
    );

    // парсинг функций без реализации
    // такие функции нужно разбить на части для корректного добавления в member, member_construct, fn блоки
    // такиим функциям будет дописано тело функции, которое содержит вызов
    // из загруженной динамической библиотеки

    // это парсинг фукнции с &mut self первым аргументом
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(&mut self $(,)? $($item:ident:$ty:ty),*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!(parse $decl $impl 
            [$($member)*]
            [$($member_construct)*]
            [$($fn)*]
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
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(&self $(,$item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $decl $impl 
            [$($member)*]
            [$($member_construct)*]
            [$($fn)*]
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
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(self $(,$item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $decl $impl 
            [$($member)*]
            [$($member_construct)*]
            [$($fn)*]
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
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident($($item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $decl $impl 
            [$($member)*]
            [$($member_construct)*]
            [$($fn)*]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name],
                [self], [], [&self,], [],
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }
        );
    );
    
    // после парсинга функций без реализации с первым аргументом:: self, &self, &mut self
    // происходит формирование блоков member, member_construct, fn
    // - member - список полей структуры, который будет отображен в блоке struct
    // - member_construct - это выражения через которые заполняются поля указателей через подгрузку динамической библиокети в конструкторе
    // - fn - список функций в impl блоке где происходит вызов функции из динамической библиокеки
    // далее из этих блоков будет составлена struct и impl блоки с полями и функциями
    // функции не имеющие реализацию будут иметь поле указатель (member) с названием этой функции
    // данный указатель будет заполняться в конструкторе через загрузку динамической библиотеки
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            [$fnattr:tt], [fn $name:ident], [$self:tt], [$($member_self:tt)*], [$($fn_decl_self:tt)*], [$($fn_call_self:tt)*], [$($item:ident : $ty:ty),*], [$(-> $ret:ty)?], $($t:tt)*
        }
    ) => (
        jude!(parse $decl $impl
            [$($member)* $name: fn($($member_self)* $($ty),*) $(-> $ret)?,]
            [ $($member_construct)* ]
            [$($fn)* $fnattr fn $name($($fn_decl_self)* $($item:$ty),*) $(-> $ret)? {
                ($self.$name)($($fn_call_self)* $($item),*)
            }]
            { $($t)* }
        );
    );
    
    // это парсинг полей структуры
    // значения которых вычисляется через выражение с блоком $body:block
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            $name:ident: $typ:ty = $body:block, $($t:tt)*
        }
    ) => (
        jude!(parse $decl $impl 
            [ $($member)* $name: $typ, ]
            [ $($member_construct)* $name = $body ]
            [ $($fn)* ]
            { $($t)* }
        );
    );

    // это парсинг полей структуры
    // значения которых вычисляется через явное назначение литерала
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            $name:ident: $typ:ty = $body:literal, $($t:tt)*
        }
    ) => (
        jude!(parse $decl $impl 
            [ $($member)* $name: $typ, ]
            [ $($member_construct)* $name = $body ]
            [ $($fn)* ]
            { $($t)* }
        );
    );

    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($member_construct:tt)*] [$($fn:tt)*] {
            $name:ident: $typ:ty, $($t:tt)*
        }
    ) => (
        jude!(parse $decl $impl 
            [$($member)* $name: $typ,]
            [$($member_construct)*]
            [$($fn)*]
            { $($t)* }
        );
    );

    (
        output [$($decls:tt)*] [$($impl:tt)*]
            [$($member:tt)*]
            [$($member_construct:tt)*]
            [$($fn:tt)*]
    )
    => (
        $crate::as_item!(
            $($decls)* {
                $($member)*
            }
        );

        $crate::as_item!(
            $($impl)* {
                $($fn)*
            }
        );
    );
);
