use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Implementation of ULID
/// - https://ulid.page
/// - https://github.com/ulid/spec
///
/// Timestamp
/// - 48 bit integer
/// - UNIX-time in milliseconds
/// - Won't run out of space 'til the year 10889 AD.
///
/// Randomness
/// - 80 bits
/// - Cryptographically secure source of randomness, if possible
///
/// ```text
///  01AN4Z07BY      79KA1307SR9X4MV3
///  
/// |----------|    |----------------|
///  Timestamp          Randomness
///    48bits             80bits
/// ```
///
/// Encoding
///
/// Crockford's Base32 is used as shown. This alphabet excludes the letters I, L, O, and U
/// to avoid confusion and abuse.
pub struct Ulid {
    id: u128,
}

const CROCKFORD_BASE32: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
impl Ulid {
    pub fn new() -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let timestamp = (timestamp_ms & 0xFFFFFFFFFFFF) << 80;

        let mut rng = rand::rng();
        let random: u128 = rng.random::<u128>() & 0xFFFFFFFFFFFFFFFFFFFF;

        let id = timestamp | random;

        Ulid { id }
    }

    pub fn to_string(&self) -> String {
        let mut result = [b'0'; 26];
        let mut value = self.id;

        for i in (0..26).rev() {
            let index = (value & 0b11111) as usize; // take 5 bits
            result[i] = CROCKFORD_BASE32[index];
            value >>= 5;
        }

        String::from_utf8(result.to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ulid_length() {
        let ulid = Ulid::new();
        let ulid_str = ulid.to_string();
        assert_eq!(ulid_str.len(), 26, "ULID length should be 26 characters");
    }

    #[test]
    fn test_ulid_uniqueness() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let count = 10_000;

        for _ in 0..count {
            let ulid = Ulid::new().to_string();
            assert!(
                set.insert(ulid),
                "Duplicate ULID detected during uniqueness test"
            );
        }
    }
}
