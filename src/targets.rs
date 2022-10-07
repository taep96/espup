//! ESP32 chip variants support.

use crate::emoji;
use log::debug;
use std::{collections::HashSet, str::FromStr};
use strum::Display;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Display)]
pub enum Target {
    /// Xtensa LX7 based dual core
    #[strum(serialize = "esp32")]
    ESP32 = 0,
    /// Xtensa LX7 based single core
    #[strum(serialize = "esp32s2")]
    ESP32S2,
    /// Xtensa LX7 based single core
    #[strum(serialize = "esp32s3")]
    ESP32S3,
    /// RISC-V based single core
    #[strum(serialize = "esp32c3")]
    ESP32C3,
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "esp32" => Ok(Target::ESP32),
            "esp32s2" => Ok(Target::ESP32S2),
            "esp32s3" => Ok(Target::ESP32S3),
            "esp32c3" => Ok(Target::ESP32C3),
            _ => Err(format!("{} Target '{}' is not supported", emoji::ERROR, s)),
        }
    }
}

/// Returns a vector of Chips from a comma or space separated string.
pub fn parse_targets(targets_str: &str) -> Result<HashSet<Target>, String> {
    debug!("{} Parsing targets: {}", emoji::DEBUG, targets_str);
    let mut targets: HashSet<Target> = HashSet::new();
    if targets_str.contains("all") {
        targets.insert(Target::ESP32);
        targets.insert(Target::ESP32S2);
        targets.insert(Target::ESP32S3);
        targets.insert(Target::ESP32C3);
        return Ok(targets);
    }
    let targets_str: HashSet<&str> = if targets_str.contains(' ') || targets_str.contains(',') {
        targets_str.split([',', ' ']).collect()
    } else {
        vec![targets_str].into_iter().collect()
    };

    for target in targets_str {
        targets.insert(FromStr::from_str(target).unwrap());
    }
    debug!("{} Parsed targets: {:?}", emoji::DEBUG, targets);
    Ok(targets)
}

#[cfg(test)]
mod tests {
    use crate::targets::{parse_targets, Target};

    #[test]
    fn test_parse_targets() {
        assert_eq!(
            parse_targets("esp32"),
            Ok([Target::ESP32].into_iter().collect())
        );
        assert_eq!(
            parse_targets("esp32,esp32s2"),
            Ok([Target::ESP32, Target::ESP32S2].into_iter().collect())
        );
        assert_eq!(
            parse_targets("esp32s3 esp32"),
            Ok([Target::ESP32S3, Target::ESP32].into_iter().collect())
        );
        assert_eq!(
            parse_targets("esp32s3,esp32,esp32c3"),
            Ok([Target::ESP32S3, Target::ESP32, Target::ESP32C3]
                .into_iter()
                .collect())
        );
        assert_eq!(
            parse_targets("all"),
            Ok([
                Target::ESP32,
                Target::ESP32S2,
                Target::ESP32S3,
                Target::ESP32C3
            ]
            .into_iter()
            .collect())
        );
    }
}
