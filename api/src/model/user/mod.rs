//! Users, bots and clients

pub mod client;

use core::str::FromStr;

use anyhow::{anyhow, Error};
use uuid::Uuid;

/// This is used to differentiate several users with a same username
///
/// It is represented with an hexadecimal format
#[derive(Debug, PartialEq, Eq)]
pub struct Discriminator([u8; 3]);

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
#[derive(Debug)]
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
