#![no_std]

#[cfg(test)]
#[allow(unused_imports)]
#[macro_use]
extern crate std;

#[cfg(not(feature = "no_std"))]
#[allow(unused_imports)]
#[macro_use]
#[doc(hidden)]
pub extern crate std as _std;

// Re-export libcore using an alias so that the macros can work without
// requiring `extern crate core` downstream.
#[doc(hidden)]
pub extern crate core as _core;

#[macro_export]
macro_rules! cenums {
    (
        $(#[$outer:meta])*
        pub struct $StructName:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Elem:ident = $value:expr;
            )+
        }
    ) => {
        __cenums! {
            $(#[$outer])*
            (pub) $StructName: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Elem = $value;
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        struct $StructName:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Elem:ident = $value:expr;
            )+
        }
    ) => {
        __cenums! {
            $(#[$outer])*
            () $StructName: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Elem = $value;
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub ($($vis:tt)+) struct $StructName:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Elem:ident = $value:expr;
            )+
        }
    ) => {
        __cenums! {
            $(#[$outer])*
            (pub ($($vis)+)) $StructName: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Elem = $value;
                )+
            }
        }
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! __cenums {
    (
        $(#[$outer:meta])*
        ($($vis:tt)*) $StructName:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $Elem:ident = $value:expr;
            )+
        }
    ) => {
        #[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
        $(#[$outer])*
        $($vis)* struct $StructName {
            value: $T,
        }

        __impl_cenums! {
            $StructName: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Elem = $value;
                )+
            }
        }
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! __impl_cenums {
    (
        $StructName:ident: $T:ty {
            $(
                $(#[$attr:ident $($args:tt)*])*
                $Elem:ident = $value:expr;
            )+
        }
    ) => {
        impl $crate::_core::fmt::Debug for $StructName {
            fn fmt(&self, f: &mut $crate::_core::fmt::Formatter) -> $crate::_core::fmt::Result {
                // This convoluted approach is to handle #[cfg]-based flag
                // omission correctly. For example it needs to support:
                //
                //    #[cfg(unix)] const A: Flag = /* ... */;
                //    #[cfg(windows)] const B: Flag = /* ... */;

                // Unconditionally define a check for every elements, even disabled
                // ones.
                #[allow(non_snake_case)]
                trait __CEnums {
                    $(
                        #[inline]
                        fn $Elem(&self) -> bool { false }
                    )+
                }

                // Conditionally override the check for just those flags that
                // are not #[cfg]ed away.
                impl __CEnums for $StructName {
                    $(
                        __impl_cenums! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Elem(&self) -> bool {
                                Self::$Elem == *self
                            }
                        }
                    )+
                }

                $(
                    if <$StructName as __CEnums>::$Elem(self) {
                        return write!(
                            f,
                            "{}::{}",
                            stringify!($StructName),
                            stringify!($Elem),
                        );
                    }
                )+
                
                write!(
                    f,
                    "{}::from({:#x})",
                    stringify!($StructName),
                    self.value,
                )
            }
        }

        #[allow(dead_code)]
        impl $StructName {
            $(
                $(#[$attr $($args)*])*
                pub const $Elem: $StructName = $StructName { value: $value };
            )+

            /// Returns the all defined values.
            #[cfg(not(feature = "no_std"))]
            #[inline]
            pub fn values() -> &'static [$StructName] {
                // See `Debug::fmt` for why this approach is taken.
                #[allow(non_snake_case)]
                trait __CEnums {
                    $(
                        #[inline]
                        fn $Elem() -> $crate::_core::option::Option<$StructName> {
                            $crate::_core::option::Option::None
                        }
                    )+
                }

                impl __CEnums for $StructName {
                    $(
                        __impl_cenums! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Elem() -> $crate::_core::option::Option<$StructName> {
                                $crate::_core::option::Option::Some(Self::$Elem)
                            }
                        }
                    )+
                }

                static mut VALUES: $crate::_std::option::Option<$crate::_std::vec::Vec<$StructName>> = $crate::_std::option::Option::None;
                static ONCE: $crate::_std::sync::Once = $crate::_std::sync::ONCE_INIT;
                unsafe {
                    ONCE.call_once(|| {
                        VALUES = Some(vec![]);
                        $(
                            if let $crate::_core::option::Option::Some(value) = <$StructName as __CEnums>::$Elem() {
                                if let Some(ref mut values) = VALUES {
                                    values.push(value);
                                }
                            }
                        )+
                    });

                    if let Some(ref values) = VALUES {
                        &values
                    } else {
                        panic!("Unreachable");
                    }
                }
            }

            #[inline]
            pub fn is_defined(&self) -> bool {
                // See `Debug::fmt` for why this approach is taken.
                #[allow(non_snake_case)]
                trait __CEnums {
                    $(
                        #[inline]
                        fn $Elem(&self) -> bool { false }
                    )+
                }

                // Conditionally override the check for just those flags that
                // are not #[cfg]ed away.
                impl __CEnums for $StructName {
                    $(
                        __impl_cenums! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Elem(&self) -> bool {
                                Self::$Elem == *self
                            }
                        }
                    )+
                }

                $(
                    if <$StructName as __CEnums>::$Elem(self) {
                        return true
                    }
                )+

                return false;
            }

            #[inline]
            pub fn value(&self) -> $T {
                self.value
            }
        }

        impl $crate::_core::convert::From<$T> for $StructName {
            #[inline]
            fn from(v: $T) -> Self {
                $StructName { value: v }
            }
        }

        /// See
        /// https://doc.rust-lang.org/std/convert/trait.Into.html#implementing-into
        impl $crate::_core::convert::From<$StructName> for $T {
            #[inline]
            fn from(v: $StructName) -> Self {
                v.value
            }
        }
    };

    // Every attribute that the user writes on a const is applied to the
    // corresponding const that we generate, but within the implementation of
    // Debug and all() we want to ignore everything but #[cfg] attributes. In
    // particular, including a #[deprecated] attribute on those items would fail
    // to compile.
    // https://github.com/bitflags/bitflags/issues/109
    //
    // Input:
    //
    //     ? #[cfg(feature = "advanced")]
    //     ? #[deprecated(note = "Use somthing else.")]
    //     ? #[doc = r"High quality documentation."]
    //     fn f() -> i32 { /* ... */ }
    //
    // Output:
    //
    //     #[cfg(feature = "advanced")]
    //     fn f() -> i32 { /* ... */ }
    (
        $(#[$filtered:meta])*
        ? #[cfg $($cfgargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        fn $($item:tt)*
    ) => {
        __impl_cenums! {
            $(#[$filtered])*
            #[cfg $($cfgargs)*]
            $(? #[$rest $($restargs)*])*
            fn $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        // $next != `cfg`
        ? #[$next:ident $($nextargs:tt)*]
        $(? #[$rest:ident $($restargs:tt)*])*
        fn $($item:tt)*
    ) => {
        __impl_cenums! {
            $(#[$filtered])*
            // $next filtered out
            $(? #[$rest $($restargs)*])*
            fn $($item)*
        }
    };
    (
        $(#[$filtered:meta])*
        fn $($item:tt)*
    ) => {
        $(#[$filtered])*
        fn $($item)*
    };
}


#[cfg(test)]
mod tests {
    cenums! {
        #[doc = "Lorem ipsum dolor sit amet, consectetur adipiscing elit"]
        struct Hoge: u32 {
            const A = 1;
            #[doc = "Duis aute irure dolor in reprehenderit in voluptate"]
            const B = 2;
            const C = 3;
        }
    }

    cenums! {
        struct Fuga: u32 {
            #[cfg(windows)]
            const A = 1;
            #[cfg(unix)]
            const B = 2;
        }
    }

    #[test]
    fn test_value() {
        assert_eq!(Hoge::A.value(), 1);
        assert_eq!(Hoge::B.value(), 2);
        assert_eq!(Hoge::C.value(), 3);
    }

    #[test]
    fn test_from_value() {
        assert_eq!(Hoge::from(1), Hoge::A);
        // or
        let a: Hoge = 1.into();
        assert_eq!(a, Hoge::A);
        //
        assert_eq!(Hoge::from(2), Hoge::B);
        assert_eq!(Hoge::from(3), Hoge::C);
    }

    #[test]
    fn test_into_value() {
        let a: u32 = Hoge::A.into();
        assert_eq!(a, 1u32);
        // or
        assert_eq!(u32::from(Hoge::A), 1u32);
        //
        let b: u32 = Hoge::B.into();
        assert_eq!(b, 2u32);
        let c: u32 = Hoge::C.into();
        assert_eq!(c, 3u32);
    }

    #[test]
    fn test_defined() {
        assert!(Hoge::is_defined(&1u32.into()));
        let d: Hoge = 4u32.into();
        assert!(!d.is_defined());

        #[cfg(windows)]
        assert!(Fuga::is_defined(&1u32.into()));
        #[cfg(not(windows))]
        assert!(!Fuga::is_defined(&1u32.into()));
        #[cfg(unix)]
        assert!(Fuga::is_defined(&2u32.into()));
        #[cfg(not(unix))]
        assert!(!Fuga::is_defined(&2u32.into()));
    }

    #[test]
    fn test_values() {
        let hoges = Hoge::values();
        assert_eq!(&[Hoge::A, Hoge::B, Hoge::C], hoges);

        let fugas = Fuga::values();
        #[cfg(windows)]
        assert_eq!(&[Fuga::A], fugas);
        #[cfg(unix)]
        assert_eq!(&[Fuga::B], fugas);
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Hoge::A), "Hoge::A");
        assert_eq!(format!("{:?}", Hoge::from(31)), "Hoge::from(0x1f)");
    }

    mod submodule {
        cenums! {
            pub struct PublicEnum: i8 {
                const X = 0;
            }
        }
        cenums! {
            struct PrivateEnum: i8 {
                const Y = 0;
            }
        }

        #[test]
        fn test_private() {
            let _ = PrivateEnum::Y;
        }
    }

    #[test]
    fn test_public() {
        let _ = submodule::PublicEnum::X;
    }

    mod t1 {
        mod foo {
            pub type Bar = i32;
        }

        cenums! {
            /// baz
            struct Enum: foo::Bar {
                const A = 0b00000001;
                #[cfg(foo)]
                const B = 0b00000010;
                #[cfg(foo)]
                const C = 0b00000010;
            }
        }
    }

    #[test]
    fn test_in_function() {
        cenums! {
           struct Enum: u8 {
                const A = 1;
                #[cfg(any())] // false
                const B = 2;
            }
        }
        assert_eq!(Enum::values(), &[Enum::A]);
        assert_eq!(format!("{:?}", Enum::A), "Enum::A");
    }

    #[test]
    fn test_deprecated() {
        cenums! {
            pub struct TestEnum: u32 {
                #[deprecated(note = "Use something else.")]
                const ONE = 1;
            }
        }
    }

    #[test]
    fn test_pub_crate() {
        mod module {
            cenums! {
                pub (crate) struct Test: u8 {
                    const FOO = 1;
                }
            }
        }

        assert_eq!(module::Test::FOO.value(), 1u8);
    }

    #[test]
    fn test_pub_in_module() {
        mod module {
            mod submodule {
                cenums! {
                    // `pub (in super)` means only the module `module` will
                    // be able to access this.
                    pub (in super) struct Test: u8 {
                        const FOO = 1;
                    }
                }
            }

            mod test {
                // Note: due to `pub (in super)`,
                // this cannot be accessed directly by the testing code.
                pub(super) fn value() -> u8 {
                    super::submodule::Test::FOO.value()
                }
            }

            pub fn value() -> u8 {
                test::value()
            }
        }

        assert_eq!(module::value(), 1)
    }
}
