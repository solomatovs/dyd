#![feature(trace_macros)]

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
            [ $name ]
            [ $vis ]
            [ $(#[$attr])* ]
            [ $(<$($lifetime),+>)* ]
            [] [] [] [] [] [] $body
        );
    );

    // на этом этапе все фуркции и поля были разбиты на блоки поэтому остался только {}
    // передаю все блоки output
    (
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {}
    ) => (
        jude!(output $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
        );
    );

    // парсинг функции с реализацией
    // такие функции обладают телом функции: $body:block
    // их передаю в исходном виде и никак не изменяем
    (
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident($($tt:tt)*) $(-> $ret:ty)? $body:block, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* $(#[$attr])* $vis $(<$($lifetime),+>)* fn $name($($tt)*) $(-> $ret)? $body ]
            [ $($fn_not_impl)* ]
            { $($t)* }
        );
    );

    // парсинг функций без реализации
    // такие функции нужно разбить на части для корректного добавления в member_impl, field_impl, fn блоки
    // такиим функциям будет дописано тело функции, которое содержит вызов
    // из загруженной динамической библиотеки

    // это парсинг фукнции с &mut self первым аргументом
    (
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(&mut self $(,)? $($item:ident:$ty:ty),*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
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
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(&self $(,$item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
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
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident(self $(,$item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
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
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis $(<$($lifetime:lifetime),+>)*
            fn $name:ident($($item:ident:$ty:ty)*) $(-> $ret:ty)?, $($t:tt)*
        }
    ) => (
        jude!( parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
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
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
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
        jude!(parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* $name: fn($($member_impl_self)* $($ty),*) $(-> $ret)?, ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* $name, ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* $fnattr fn $name($($fn_struct_decl_self)* $($item:$ty),*) $(-> $ret)? {
                ($self.$name)($($fn_call_self)* $($item),*)
            }]
            { $($t)* }
        );
    );

    // это парсинг полей структуры
    // значения которых вычисляется через выражение с блоком $body:block
    (
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis
            $name:ident: $(<$($lifetime:lifetime),+>)* $typ:ty = $body:block, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* $(#[$attr])* $vis $name: $(<$($lifetime),+>)* $typ, ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* $name: $body, ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
            { $($t)* }
        );
    );

    // это парсинг полей структуры
    // значения которых вычисляется через явное назначение литерала
    (
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis
            $name:ident: $(<$($lifetime:lifetime),+>)* $typ:ty = $body:literal, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* $(#[$attr])* $vis $name: $(<$($lifetime),+>)* $typ, ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* $name: $body, ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
            { $($t)* }
        );
    );

    // это парсинг полей структуры
    // значения которых вычисляется через явное назначение expr
    (
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis
            $name:ident: $(<$($lifetime:lifetime),+>)* $typ:ty = $($body:expr)+, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* $(#[$attr])* $vis $name: $(<$($lifetime),+>)* $typ, ]
            [ $($member_not_impl)* ]
            [ $($field_impl)* $name: $($body)+, ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
            { $($t)* }
        );
    );

    (
        parse $struct_name:tt $struct_vis:tt $struct_attr:tt $struct_lifetime:tt
        [ $($member_impl:tt)* ]
        [ $($member_not_impl:tt)* ]
        [ $($field_impl:tt)* ]
        [ $($field_not_impl:tt)* ]
        [ $($fn_impl:tt)* ]
        [ $($fn_not_impl:tt)* ]
        {
            $(#[$attr:meta])*
            $vis:vis
            $name:ident: $(<$($lifetime:lifetime),+>)* $typ:ty, $($t:tt)*
        }
    ) => (
        jude!(parse $struct_name $struct_vis $struct_attr $struct_lifetime
            [ $($member_impl)* ]
            [ $($member_not_impl)* $(#[$attr])* $vis $name: $(<$($lifetime),+>)* $typ, ]
            [ $($field_impl)* ]
            [ $($field_not_impl)* ]
            [ $($fn_impl)* ]
            [ $($fn_not_impl)* ]
            { $($t)* }
        );
    );

    // если нет полей и методов без реализации
    // то создаем структуру без load_from_lib
    (
        output
            [ $struct_name:tt ]
            [ $struct_vis:tt ]
            [ $(#[$struct_attr:meta])* ]
            [ $(<$($struct_lifetime:lifetime),+>)* ]
            [ $($member_impl:tt)* ]
            [ ]
            [ $($field_impl:tt)* ]
            [ ]
            [ $($fn_impl:tt)* ]
            [ ]
    )
    => (
        $crate::as_item!(
            $(#[$struct_attr])*
            $struct_vis struct $struct_name $(<$($struct_lifetime),+>)* {
                $($member_impl)*
            }
        );

        $crate::as_item!(
            impl $(<$($struct_lifetime),+>)* $struct_name $(<$($struct_lifetime),+>)* {
                $($fn_impl)*
            }
        );

        $crate::as_item!(
            impl $(<$($struct_lifetime),+>)* std::default::Default for $struct_name $(<$($struct_lifetime),+>)* {
                fn default() -> Self {
                    Self {
                        $($field_impl)*
                    }
                }
            }
        );
    );

    (
        output
            [ $struct_name:tt ]
            [ $struct_vis:tt ]
            [ $(#[$struct_attr:meta])* ]
            [ $(<$($struct_lifetime:lifetime),+>)* ]
            [ $($member_impl:tt)* ]
            [ $($member_not_impl:tt)* ]
            [ $($field_impl:tt)* ]
            [ $($field_not_impl:tt, )* ]
            [ $($fn_impl:tt)* ]
            [ $($fn_not_impl:tt)* ]
    )
    => (
        $crate::as_item!(
            $(#[$struct_attr])*
            $struct_vis struct $struct_name $(<$($struct_lifetime),+>)* {
                $($member_impl)*
                $($member_not_impl)*
                __from_file: std::ffi::OsString,
                __from_lib: std::sync::Arc<libloading::Library>,
                __modified: std::time::SystemTime,
            }
        );

        $crate::as_item!(
            impl $(<$($struct_lifetime),+>)* $struct_name $(<$($struct_lifetime),+>)* {
                $($fn_impl)*
            }
        );

        $crate::as_item!(
            impl $(<$($struct_lifetime),+>)* $struct_name $(<$($struct_lifetime),+>)* {
                $($fn_not_impl)*

                fn load_from_lib(file_path: std::ffi::OsString) -> Result<Self, libloading::Error> { //Result<Self, $crate::JudeError> {
                    let lib = unsafe {
                        libloading::Library::new(&file_path)
                    }?;

                    let modified = std::fs::metadata(&file_path).unwrap();
                    let modified = modified.modified().unwrap();

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
                        __from_file: file_path,
                        __from_lib: std::sync::Arc::new(lib),
                        __modified: modified,
                    };

                    Ok(res)
                }

                fn reload_lib(&mut self) -> Result<(), libloading::Error> {
                    
                    let from_file = self.__from_file.clone();
                    let lib = unsafe {
                        libloading::Library::new(&from_file)
                    }?;

                    let modified = std::fs::metadata(&from_file).unwrap();
                    let modified = modified.modified().unwrap();

                    $(
                        self.$field_not_impl = {
                            let symbol = unsafe {
                                lib.get(stringify!($field_not_impl).as_bytes())
                            }?;

                            *symbol
                        };
                    )*

                    self.__from_lib = std::sync::Arc::new(lib);
                    self.__from_file = from_file;
                    self.__modified = modified;
    
                    Ok(())
                }

                fn changed(&self) -> Result<bool, std::io::Error> {
                    let modified = std::fs::metadata(&self.__from_file)?;
                    let modified = modified.modified()?;

                    match modified.duration_since(self.__modified) {
                        Ok(x) => Ok(x.is_zero()),
                        Err(e) => Ok(false),
                    }
                }
            }
        );
    );
);
