
// tests5.rs
//
// An `unsafe` in Rust serves as a contract.

// SAFETY:
// The caller must ensure that `address` points to a valid,
// properly aligned, writable u32 value with unique access.
unsafe fn modify_by_address(address: usize) {
    // SAFETY:
    // The caller guarantees that the address represents
    // a valid and uniquely accessible u32 value.
    unsafe {
        let ptr = address as *mut u32;
        *ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;

        // SAFETY: The address is valid and points to
        // a uniquely accessible local u32 variable.
        unsafe {
            modify_by_address(&mut t as *mut u32 as usize);
        }

        assert!(t == 0xAABBCCDD);
    }
}
