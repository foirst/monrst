//! Everything linked with the protocol used through websocket streams

use core::fmt;
use core::str::FromStr;

use anyhow::{anyhow, Error};
use spin::Lazy;

/// Partial implementation of the Semantic Versioning 2.0.0 ([see here for more details](https://semver.org/))
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    /// Major version, increasing with incompatible API changes
    pub major: usize,

    /// Minor version, adding new functionalities
    pub minor: usize,

    /// Patch version, making backward compatible bug fixes
    pub patch: usize,

    /// Additional labels
    pub extra_labels: Option<String>,
}

impl fmt::Display for Version {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.extra_labels {
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
            Some(label) => write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, label),
        }
    }
}

impl FromStr for Version {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted_value = s.split('.');
        let Some(major) = splitted_value.next() else { return Err(anyhow!("Could not retrieve the major version from {}", s)) };
        let Some(minor) = splitted_value.next() else { return Err(anyhow!("Could not retrieve the minor version from {}", s)) };
        let Some(patch_and_labels) = splitted_value.next() else { return Err(anyhow!("Could not retrieve the patch version and/or the extra label from {}", s)) };
        let mut splitted_patch_and_labels = patch_and_labels.split('-');
        let Some(patch) = splitted_patch_and_labels.next() else { return Err(anyhow!("Could not retrieve the patch version from {}", s)) };
        let extra_labels = splitted_patch_and_labels.next().map(ToOwned::to_owned);

        Ok(Self {
            major: major.parse::<usize>()?,
            minor: minor.parse::<usize>()?,
            patch: patch.parse::<usize>()?,
            extra_labels,
        })
    }
}

impl Version {
    /// Indicates if two given [Version] are compatible
    ///
    /// Two versions are compatible if they have common major versions different than 0, or if their major version is equal to 0
    /// and have two minor versions equal
    #[inline]
    #[must_use]
    pub const fn are_compatible(version1: &Self, version2: &Self) -> bool {
        version1.major == version2.major && (version1.major != 0 || version1.minor == version2.minor)
    }
}

/// Protocol version
///
/// It is always equal to `Monrst`'s version
pub static VERSION: Lazy<Version> = Lazy::new(|| {
    Version::from_str(env!("CARGO_PKG_VERSION"))
        .unwrap_or_else(|_| unreachable!("The monrst package always follows the Semantic Versioning standard"))
});

/// Kinds of format supported by the protocol
#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    /// Bytes-base communication
    Binary,

    /// JSON-based communication
    Json,
}

impl FromStr for Format {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "binary" | "BINARY" => Ok(Self::Binary),
            "json" | "JSON" => Ok(Self::Json),
            other => Err(anyhow!("{other} does not correspond to any known format")),
        }
    }
}

/// Protocol configuration
#[derive(Debug)]
pub struct Configuration {
    /// Format used
    pub format: Format,
}

#[cfg(test)]
mod test {
    use super::Version;

    #[test]
    fn versions() {
        assert_eq!("1.2.3".parse::<Version>().unwrap(), Version {
            major: 1,
            minor: 2,
            patch: 3,
            extra_labels: None
        });
        assert_eq!("4.5.6-beta".parse::<Version>().unwrap(), Version {
            major: 4,
            minor: 5,
            patch: 6,
            extra_labels: Some("beta".to_owned())
        });

        assert!(Version::are_compatible(
            &Version {
                major: 0,
                minor: 1,
                patch: 2,
                extra_labels: None
            },
            &Version {
                major: 0,
                minor: 1,
                patch: 5,
                extra_labels: Some("beta".to_owned())
            }
        ));
        assert!(Version::are_compatible(
            &Version {
                major: 1,
                minor: 2,
                patch: 3,
                extra_labels: None
            },
            &Version {
                major: 1,
                minor: 4,
                patch: 5,
                extra_labels: Some("alpha".to_owned())
            }
        ));
        assert!(!Version::are_compatible(
            &Version {
                major: 0,
                minor: 1,
                patch: 2,
                extra_labels: None
            },
            &Version {
                major: 0,
                minor: 3,
                patch: 4,
                extra_labels: None
            }
        ));
        assert!(!Version::are_compatible(
            &Version {
                major: 2,
                minor: 2,
                patch: 3,
                extra_labels: None
            },
            &Version {
                major: 1,
                minor: 4,
                patch: 5,
                extra_labels: Some("alpha".to_owned())
            }
        ));
    }
}
