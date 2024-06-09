use std::fmt;

pub struct HexAddress(pub u64);

impl fmt::Display for HexAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:016X} ({} bytes)", self.0, self.0)
    }
}
