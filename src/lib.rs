//! Create and use UUID's
// #![cfg_attr(not(test), no_std)]

/// A 16 byte with the UUID.
pub type Bytes = [u8; 16];

/// UUID Variants
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Variant {
    /// Reserved for NCS backward compatibility.
    Ncs,

    /// RFC 4122 conforming UUID's.
    Rfc4122,

    /// Reserved for legacy Microsoft backward compatibility.
    Microsoft,

    /// Reserved for the future.
    Reserved,
}

/// UUID Version
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Version {
    /// Version 1, time based.
    Time,

    /// Version 2, DCE Security.
    Dce,

    /// Version 3, MD5 name based.
    Md5,

    /// Version 4, random.
    Random,

    /// Version 5, SHA-1 name based.
    Sha1,
}

/// Universally Unique Identifier, or UUID.
///
/// This type is `repr(transparent)` and guaranteed to have the same layout
/// as `[u8; 16]`.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Uuid(Bytes);

impl Uuid {
    pub fn nil() -> Self {
        Uuid(Bytes::default())
    }

    pub fn from_bytes(bytes: Bytes) -> Self {
        Self(bytes)
    }

    pub fn to_bytes(self) -> Bytes {
        self.0
    }

    pub fn is_nill(self) -> bool {
        self.0 == Self::nil().0
    }

    pub fn variant(self) -> Variant {
        let b = self.0[8];
        match b {
            // MSB0 0
            x if x & (1 << 7) == 0 => Variant::Ncs,
            // MSB0 1 MSB1 0
            x if x & ((1 << 7) | !(1 << 6)) != 0 => Variant::Ncs,
            _ => panic!(),
        }
        // // b.tole
        // dbg!(b);
        // dbg!(b << 0);
        // dbg!(b << 7);
        // dbg!(b << 6);
        // dbg!((b << 5));
        // dbg!((b << 5).to_be());
        // todo!()
    }

    pub fn version(self) -> Version {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _UUID_V4: &str = "662aa7c7-7598-4d56-8bcc-a72c30f998a2";
    const RAW: [u8; 16] = [
        102, 42, 167, 199, 117, 152, 77, 86, 139, 204, 167, 44, 48, 249, 152, 162,
    ];

    #[test]
    fn info() {
        let uuid = Uuid::from_bytes(RAW);
        // dbg!(uuid);
        dbg!(uuid.variant());
        assert_eq!(uuid.version(), Version::Random);
        todo!("test")
    }
}
