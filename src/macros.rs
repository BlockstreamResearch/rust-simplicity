#[macro_export]
macro_rules! decode_bits {
    ($bits:ident, {}) => {
        Err($crate::decode::Error::InvalidJet.into())
    };
    ($bits:ident, {$jet:path}) => {
        Ok($jet)
    };
    ($bits:ident, { 0 => $false_branch:tt, 1 => $true_branch:tt }) => {
        match $bits.next() {
            None => Err($crate::decode::Error::EndOfStream.into()),
            Some(false) => decode_bits!($bits, $false_branch),
            Some(true) => decode_bits!($bits, $true_branch),
        }
    };
}

/// Copied from rust-miniscript
///
/// A macro that implements serde serialization and deserialization using the
/// `fmt::Display` and `str::FromStr` traits.
#[cfg(feature = "elements")]
macro_rules! serde_string_impl_pk {
    ($name:ident, $expecting:expr $(, $gen:ident; $gen_con:ident)*) => {
        #[cfg(feature = "serde")]
        impl<'de, Pk $(, $gen)*> $crate::serde::Deserialize<'de> for $name<Pk $(, $gen)*>
        where
            Pk: crate::miniscript::ToPublicKey + core::str::FromStr,
            Pk::Sha256: core::str::FromStr,
            Pk::Hash256: core::str::FromStr,
            Pk::Ripemd160: core::str::FromStr,
            Pk::Hash160: core::str::FromStr,
            <Pk as core::str::FromStr>::Err: core::fmt::Display,
            <<Pk as crate::miniscript::MiniscriptKey>::Sha256 as core::str::FromStr>::Err:
                core::fmt::Display,
            <<Pk as crate::miniscript::MiniscriptKey>::Hash256 as core::str::FromStr>::Err:
                core::fmt::Display,
            <<Pk as crate::miniscript::MiniscriptKey>::Ripemd160 as core::str::FromStr>::Err:
                core::fmt::Display,
            <<Pk as crate::miniscript::MiniscriptKey>::Hash160 as core::str::FromStr>::Err:
                core::fmt::Display,
            $($gen : $gen_con,)*
        {
            fn deserialize<D>(deserializer: D) -> Result<$name<Pk $(, $gen)*>, D::Error>
            where
                D: $crate::serde::de::Deserializer<'de>,
            {
                use core::fmt::{self, Formatter};
                use core::marker::PhantomData;
                use core::str::FromStr;

                #[allow(unused_parens)]
                struct Visitor<Pk $(, $gen)*>(PhantomData<(Pk $(, $gen)*)>);
                impl<'de, Pk $(, $gen)*> $crate::serde::de::Visitor<'de> for Visitor<Pk $(, $gen)*>
                where
                    Pk: crate::miniscript::ToPublicKey + core::str::FromStr,
                    Pk::Sha256: core::str::FromStr,
                    Pk::Hash256: core::str::FromStr,
                    Pk::Ripemd160: core::str::FromStr,
                    Pk::Hash160: core::str::FromStr,
                    <Pk as core::str::FromStr>::Err: core::fmt::Display,
                    <<Pk as crate::miniscript::MiniscriptKey>::Sha256 as core::str::FromStr>::Err:
                        core::fmt::Display,
                    <<Pk as crate::miniscript::MiniscriptKey>::Hash256 as core::str::FromStr>::Err:
                        core::fmt::Display,
                    <<Pk as crate::miniscript::MiniscriptKey>::Ripemd160 as core::str::FromStr>::Err:
                        core::fmt::Display,
                    <<Pk as crate::miniscript::MiniscriptKey>::Hash160 as core::str::FromStr>::Err:
                        core::fmt::Display,
                    $($gen: $gen_con,)*
                {
                    type Value = $name<Pk $(, $gen)*>;

                    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                        formatter.write_str($expecting)
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: $crate::serde::de::Error,
                    {
                        $name::from_str(v).map_err(E::custom)
                    }

                    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                    where
                        E: $crate::serde::de::Error,
                    {
                        self.visit_str(v)
                    }

                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: $crate::serde::de::Error,
                    {
                        self.visit_str(&v)
                    }
                }

                deserializer.deserialize_str(Visitor(PhantomData))
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, Pk $(, $gen)*> $crate::serde::Serialize for $name<Pk $(, $gen)*>
        where
            Pk: crate::miniscript::ToPublicKey,
            $($gen: $gen_con,)*
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                serializer.collect_str(&self)
            }
        }
    };
}
