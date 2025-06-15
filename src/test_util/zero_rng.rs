use rand::RngCore;

pub struct ZeroRng();

impl RngCore for ZeroRng {
    fn next_u32(&mut self) -> u32 {
        return 0;
    }

    fn next_u64(&mut self) -> u64 {
        return 0;
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        for slot in dst {
            *slot = 0
        }
    }
}

impl ZeroRng {
    pub fn new() -> ZeroRng {
        return ZeroRng();
    }
}
