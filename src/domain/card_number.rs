//! src/domain/user_name.rs

use std::str::FromStr;

use rand::distributions::Uniform;
use rand::thread_rng;
use rand::Rng;
use serde::{de::Visitor, Deserialize, Serialize};

/// This type guarantees us that `UserName` is properly formed.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct CardNumber(String);

impl CardNumber {
    /// Returns an instance of `CardNumber` if the input satisfies
    /// our validation constraints on card numbers.
    pub fn parse(card: &str) -> Result<CardNumber, anyhow::Error> {
        if !card.chars().all(char::is_numeric) {
            return Err(anyhow::anyhow!("Card should contain only numerics"));
        }
        if card.len() != 16 {
            return Err(anyhow::anyhow!("Card should contain 16 numbers"));
        }
        Ok(CardNumber(String::from(card)))
    }

    pub fn generate() -> Self {
        let mut rng = thread_rng();
        CardNumber(
            // ASCII table contains number symbols from 48 to 57 cells
            std::iter::repeat_with(|| rng.sample(Uniform::new(48, 57)))
                .map(char::from)
                .take(16)
                .collect(),
        )
    }
}

impl AsRef<str> for CardNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for CardNumber {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CardNumber::parse(s)
    }
}

struct CardNumberVisitor;

impl<'de> Visitor<'de> for CardNumberVisitor {
    type Value = CardNumber;
    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "a valid card number, 16 digits")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<CardNumber>()
            .map_err(|e| serde::de::Error::custom(e))
    }
}

impl<'de> Deserialize<'de> for CardNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CardNumberVisitor)
    }
}

// ───── Unit tests ───────────────────────────────────────────────────────── //

#[cfg(test)]
mod tests {
    use super::CardNumber;

    #[test]
    fn test_correct_card_number() {
        let card = CardNumber::parse("1234123412341234");
        assert!(card.is_ok())
    }

    #[test]
    fn test_card_contains_non_numeric_symbols() {
        let card = CardNumber::parse("123412341234123f");
        assert!(card.is_err())
    }

    #[test]
    fn test_card_has_bad_length() {
        let card = CardNumber::parse("1234123412341");
        assert!(card.is_err())
    }

    #[test]
    fn generation_success() {
        for _ in 0..100 {
            let card = CardNumber::generate();
            assert!(CardNumber::parse(&card.0).is_ok())
        }
    }
}
