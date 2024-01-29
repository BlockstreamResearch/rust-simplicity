// SPDX-License-Identifier: CC0-1.0

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

#[macro_export]
macro_rules! impl_serde_string {
    ($name:ident) => {
        #[cfg(feature = "serde")]
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                <$name as std::str::FromStr>::from_str(&s).map_err(serde::de::Error::custom)
            }
        }
    };
}
