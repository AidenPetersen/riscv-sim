/// Trait for memory and caches to implement load and store operations.
pub trait Storage {
    /// Loads a byte from storage
    fn load_b(&mut self, addr: u32) -> u32;
    /// Loads a half-word from storage
    fn load_h(&mut self, addr: u32) -> u32;
    /// Loads a word from storage
    fn load_w(&mut self, addr: u32) -> u32;

    /// Stores a byte to storage
    fn store_b(&mut self, addr: u32, val: u32);
    /// Stores a half-word to storage
    fn store_h(&mut self, addr: u32, val: u32);
    /// Stores a word to storage
    fn store_w(&mut self, addr: u32, val: u32);
}
