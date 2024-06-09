const SAFE: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

pub fn format(prefix: &str, size: usize) -> String {
    format!("{}_{}", prefix, nanoid::nanoid!(size, &SAFE))
}

#[macro_export]
macro_rules! prefixid {
    ($prefix:expr) => {
        $crate::format($prefix, 21)
    };

    ($prefix:expr, $size:expr) => {
        $crate::format($prefix, $size)
    };
}

#[derive(Debug, thiserror::Error)]
pub enum CreateIdError {
    #[error("invalid size")]
    InvalidSize,

    #[error("invalid prefix")]
    InvalidPrefix,
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "serde")]
macro_rules! impl_serde {
    ($name:ident) => {
        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s: &str = serde::Deserialize::deserialize(deserializer)?;
                let id = s.parse().map_err(serde::de::Error::custom)?;
                Ok(id)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "serde"))]
macro_rules! impl_serde {
    ($name:ident) => {};
}

/// Construct a new type that represents an ID with a prefix.
///
///
#[macro_export]
macro_rules! create_id {
    ($name:ident, $prefix:expr) => {
        create_id!($name, $prefix, 21);
    };
    ($name:ident, $prefix:expr, $size:expr) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(smol_str::SmolStr);

        impl $name {
            pub fn new() -> Self {
                Self(smol_str::SmolStr::new(&crate::prefixid!($prefix, $size)))
            }

            #[inline]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl ::core::convert::Into<String> for $name {
            fn into(self) -> String {
                self.0.into()
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::CreateIdError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let total_len = $size + $prefix.len() + 1;
                if s.len() != total_len {
                    return Err(crate::CreateIdError::InvalidSize);
                }

                if !s.starts_with($prefix) {
                    return Err(crate::CreateIdError::InvalidPrefix);
                }

                Ok(Self(smol_str::SmolStr::new(s)))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.0.as_str()
            }
        }

        crate::impl_serde!($name);
    };
}

#[cfg(test)]
mod test_macros {
    use super::*;

    #[test]
    fn simple() {
        let id: String = prefixid!("im");

        assert_eq!(id.len(), 24);
        assert_eq!(&id[..3], "im_");
    }

    #[test]
    fn size() {
        let id: String = prefixid!("im", 10);

        assert_eq!(id.len(), 13);
        assert_eq!(&id[..3], "im_");
    }

    #[test]
    fn simple_with_prefix() {
        create_id!(ImId, "im");

        let id = ImId::new();

        assert_eq!(id.len(), 24);
        assert_eq!(&id[..3], "im_");
    }

    #[test]
    fn simple_with_prefix_and_size() {
        create_id!(ImId, "im", 10);

        let id = ImId::new();

        assert_eq!(id.len(), 13);
        assert_eq!(&id[..3], "im_");
    }

    #[test]
    #[should_panic]
    fn throw_invalid_size() {
        create_id!(ImId, "im", 10);
        let id: ImId = "im_123456789".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn throw_invalid_prefix() {
        create_id!(ImId, "im", 10);
        let id: ImId = "in_1234567890".parse().unwrap();
    }

    #[test]
    fn test_as_str() {
        create_id!(ImId, "im", 10);

        let id = ImId::new();

        assert_eq!(id.as_str(), &id[..]);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_serialize() {
        create_id!(ImId, "im", 10);

        let id = ImId::new();

        let serialized = serde_json::to_string(&id).unwrap();

        assert_eq!(serialized, format!("\"{}\"", id));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde_deserialize() {
        create_id!(ImId, "im", 10);
        let as_str = "\"im_1234567890\"";

        let id: ImId = serde_json::from_str(as_str).unwrap();

        assert_eq!(id.as_str(), "im_1234567890");
    }
}
