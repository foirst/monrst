//! Users, bots and clients

pub mod client;

use core::str::FromStr;

use anyhow::{anyhow, Error};
use derive_more::{Deref, DerefMut};
use rand::random;
use uuid::Uuid;

/// This is used to differentiate several users with a same username
///
/// It is represented with an hexadecimal format
#[derive(Debug, Clone, Deref, DerefMut, PartialEq, Eq)]
pub struct Discriminator(pub [u8; 3]);

impl Discriminator {
    /// Creates a new discriminator
    #[inline]
    #[must_use]
    pub const fn new(discriminator: [u8; 3]) -> Self {
        Self(discriminator)
    }

    /// Generates a new random discriminator
    #[inline]
    #[must_use]
    pub fn generate() -> Self {
        Self(random())
    }
}

impl FromStr for Discriminator {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 6 {
            (0..s.len())
                .step_by(2)
                .map(|i| s.get(i..i + 2).and_then(|sub| u8::from_str_radix(sub, 16).ok()))
                .collect::<Option<Vec<u8>>>()
                .map_or_else(
                    || Err(anyhow!("")),
                    // SAFETY: it is checked that the size of `s` is 6
                    |discriminator| Ok(Self(unsafe { discriminator.as_chunks::<3>().0.get_unchecked(0) }.to_owned())),
                )
        } else {
            Err(anyhow!(""))
        }
    }
}

/// Main structure for users
///
/// A user corresponds to an account owned by a physical or moral person: it is not a bot
#[derive(Debug, Clone)]
pub struct User {
    /// Unique identifier
    pub uuid: Uuid,

    /// Username
    pub username: String,

    /// Discriminator
    pub discriminator: Discriminator,

    /// Is this user currently online ?
    pub online: bool,
}

impl User {
    /// Creates a new user
    #[inline]
    #[must_use]
    pub fn new(username: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            username: username.to_owned(),
            discriminator: Discriminator::generate(),
            online: false,
        }
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use super::Discriminator;

    #[test]
    fn discriminator() {
        assert_eq!(Discriminator([0x12_u8, 0xA0_u8, 0xBC_u8]), Discriminator::from_str("12A0BC").unwrap());
        assert!(Discriminator::from_str("1234567").is_err());
        assert!(Discriminator::from_str("12CD5Z").is_err());
    }
}
