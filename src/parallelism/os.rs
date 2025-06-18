use std::{num::NonZero, thread::available_parallelism};

use log::warn;

pub fn get_parallelism_from_os() -> NonZero<usize> {
    match available_parallelism() {
        Ok(val) => val,
        Err(e) => {
            warn!(
                "Unable to determine the number of CPU's in the system. Disabling parallelism. Reason: {}",
                e
            );
            NonZero::new(1).unwrap()
        }
    }
}
