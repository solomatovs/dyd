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
pub enum DydError {
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

impl From<libloading::Error> for DydError {
    fn from(item: libloading::Error) -> Self {
        DydError::FailedToLoadLib(item)
    }
}

impl<T> From<TryLockError<T>> for DydError {
    fn from(item: TryLockError<T>) -> Self {
        match item {
            TryLockError::WouldBlock => DydError::WouldBlock("lib".into()),
            TryLockError::Poisoned(x) => DydError::PoisonedLock(x.to_string(), "lib".into()),
        }
    }
}

#[macro_export]
macro_rules! as_item( ($i:item) => ($i) );

#[macro_export]
macro_rules! dyd(
    (
        $(#[$attr:meta])*
        $vis:vis $(<$($lifetime:lifetime),+>)*
        struct $name:ident $body:tt
    ) => (
        dyd!(parse [$(#[$attr])* $vis $(<$($lifetime),+>)* struct $name] [impl $name] [] [] [] $body);
    );
    
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($fn_member:tt)*] [$($fn:tt)*] {}
    ) => (
        dyd!(output $decl $impl
            [$($member)*]
            [$($fn_member)*]
            [$($fn)*]
        );
    );

    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($fn_member:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(&mut self $(,)? $($item:ident:$ty:ty),*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        dyd!(parse $decl $impl 
            [$($member)*]
            [$($fn_member)*]
            [$($fn)*]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name], 
                [self], [&mut Self,], [&mut self,], [self,], 
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }
        );
    );

    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($fn_member:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(&self $(,$item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        dyd!( parse $decl $impl 
            [$($member)*]
            [$($fn_member)*]
            [$($fn)*]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name],
                [self], [&Self,], [&self,], [self,],
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }

        );
    );

    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($fn_member:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(self $(,$item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        dyd!( parse $decl $impl 
            [$($member)*]
            [$($fn_member)*]
            [$($fn)*]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name],
                [self], [Self,], [self,], [self,],
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }

        );
    );
    
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($fn_member:tt)*] [$($fn:tt)*] {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident($($item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        dyd!( parse $decl $impl 
            [$($member)*]
            [$($fn_member)*]
            [$($fn)*]
            {
                [$(#[$attr])* $vis $(<$($lifetime),+>)*],
                [fn $name],
                [self], [], [&self,], [],
                [$($item:$ty),*], [$(-> $ret)?], $($t)*
            }

        );
    );
    
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($fn_member:tt)*] [$($fn:tt)*] {
            [$fnattr:tt], [fn $name:ident], [$self:tt], [$($member_self:tt)*], [$($fn_decl_self:tt)*], [$($fn_call_self:tt)*], [$($item:ident : $ty:ty),*], [$(-> $ret:ty)?], $($t:tt)*
        }
    ) => (
        dyd!(parse $decl $impl
            [$($member)* $name: fn($($member_self)* $($ty),*) $(-> $ret)?,]
            [ $($fn_member)*, ]
            [$($fn)* $fnattr fn $name($($fn_decl_self)* $($item:$ty),*) $(-> $ret)? {
                ($self.$name)($($fn_call_self)* $($item),*)
            }]
            { $($t)* }
        );
    );
    
    (
        parse $decl:tt $impl:tt [$($member:tt)*] [$($fn_member:tt)*] [$($fn:tt)*] {
            $name:ident: $typ:ty, $($t:tt)*
        }
    ) => (
        dyd!(parse $decl $impl 
            [$($member)* $name: $typ,]
            [$($fn_member)*]
            [$($fn)*]
            { $($t)* }
        );
    );

    (
        output [$($decls:tt)*] [$($impl:tt)*]
            [$($member:tt)*]
            [$($fn_member:tt)*]
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
