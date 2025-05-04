use once_cell::sync::Lazy;
use rand::Rng;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Implementation of ObjectId
/// - https://www.mongodb.com/docs/manual/reference/bson-types/#objectid
///
/// ObjectIds are small, likely unique, fast to generate, and ordered. ObjectId
/// values are 12 bytes in length, consisting of:
/// - A 4-byte timestamp, representing the ObjectId's creation, measured in seconds since the Unix epoch.
/// - A 5-byte random value generated once per client-side process. This random value is unique to the machine and process. If the process restarts or the primary node of the process changes, this value is re-generated.
/// - A 3-byte incrementing counter per client-side process, initialized to a random value. The counter resets when a process restarts.
pub struct ObjectId {
    id: [u8; 12],
}

// Global random + counter, initialized once
static MACHINE_ID: Lazy<[u8; 5]> = Lazy::new(|| {
    let mut rng = rand::rng();
    rng.random()
});
static COUNTER: Lazy<AtomicU32> = Lazy::new(|| {
    let init = rand::rng().random_range(0..0xFFFFFF);
    AtomicU32::new(init)
});

impl ObjectId {
    pub fn new() -> Self {
        let mut id = [0u8; 12];

        // 4-byte timestamp (seconds)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;
        id[0..4].copy_from_slice(&timestamp.to_be_bytes());

        // 5-byte machine/process unique random
        id[4..9].copy_from_slice(&MACHINE_ID[..]);

        // 3-byte counter (incrementing)
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst) & 0xFFFFFF; // keep 3 bytes
        id[9..12].copy_from_slice(&counter.to_be_bytes()[1..]); // use only last 3 bytes

        ObjectId { id }
    }

    pub fn to_string(&self) -> String {
        self.id.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_object_id_length() {
        let obj_id = ObjectId::new();
        let hex = obj_id.to_string();
        assert_eq!(
            hex.len(),
            24,
            "ObjectId hex string should be 24 chars (12 bytes)"
        );
    }

    #[test]
    fn test_object_id_order() {
        let id1 = ObjectId::new().to_string();
        let id2 = ObjectId::new().to_string();

        assert!(
            id1 < id2,
            "ObjectIds should be ordered; expected {} < {}",
            id1,
            id2
        );
    }

    #[test]
    fn test_object_id_uniqueness() {
        let mut set = HashSet::new();
        let count = 10_000;

        for _ in 0..count {
            let obj_id = ObjectId::new().to_string();
            assert!(
                set.insert(obj_id),
                "Duplicate ObjectId detected during uniqueness test"
            );
        }
    }
}
