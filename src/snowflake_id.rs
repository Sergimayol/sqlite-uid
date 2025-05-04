use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static SEQUENCE: Lazy<AtomicU16> = Lazy::new(|| AtomicU16::new(0));
static LAST_TIMESTAMP: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

/// | Bits | Field                | Description                                         |
/// | ---- | -------------------- | --------------------------------------------------- |
/// | 1    | sign (unused)        | always 0                                            |
/// | 41   | timestamp            | milliseconds since a custom epoch                   |
/// | 10   | machine ID / node ID | identifies the node generating the ID               |
/// | 12   | sequence             | incremental counter within the same millisecond     |
pub struct SnowflakeId {
    id: u64,
}

impl SnowflakeId {
    pub fn new(machine_id: u16, my_epoch: u64) -> Self {
        let mut timestamp = current_millis();

        let mut last_ts = LAST_TIMESTAMP.lock().unwrap();
        let sequence = if *last_ts == timestamp {
            let seq = SEQUENCE.fetch_add(1, Ordering::SeqCst) & 0x0FFF; // 12 bits
            if seq == 0 {
                while timestamp <= *last_ts {
                    timestamp = current_millis();
                }
                SEQUENCE.store(0, Ordering::SeqCst);
            }
            seq
        } else {
            SEQUENCE.store(0, Ordering::SeqCst);
            0
        };

        *last_ts = timestamp;

        let id = ((timestamp - my_epoch) & 0x1FFFFFFFFFF) << 22 // 41 bits
            | ((machine_id as u64 & 0x3FF) << 12) // 10 bits
            | (sequence as u64 & 0xFFF); // 12 bits

        SnowflakeId { id }
    }

    pub fn to_string(&self) -> String {
        self.id.to_string()
    }
}

fn current_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    const MACHINE_ID: u16 = 1;
    const EPOCH: u64 = 1_700_000_000_000;

    #[test]
    fn test_snowflake_id_length() {
        let id = SnowflakeId::new(MACHINE_ID, EPOCH);
        assert!(id.id <= u64::MAX, "Snowflake ID must fit in 64 bits");
    }

    #[test]
    fn test_snowflake_id_order() {
        let id1 = SnowflakeId::new(MACHINE_ID, EPOCH).id;
        let id2 = SnowflakeId::new(MACHINE_ID, EPOCH).id;
        assert!(
            id1 < id2,
            "IDs should be ordered; expected {} < {}",
            id1,
            id2
        );
    }

    #[test]
    fn test_snowflake_id_uniqueness() {
        let mut seen = HashSet::new();
        let count = 10_000;

        for _ in 0..count {
            let id = SnowflakeId::new(MACHINE_ID, EPOCH).id;
            assert!(seen.insert(id), "Duplicate Snowflake ID detected: {}", id);
        }
    }
}
