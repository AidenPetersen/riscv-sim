/// Trait all component should implement to cause updates upon clock cycles.
pub trait Component {
    /// Simulates clock cycle
    fn cycle(&mut self);
}