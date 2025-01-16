pub struct UIDGenerator(pub u32);
impl UIDGenerator {
    pub fn generate(&mut self) -> u32 {
        self.0 += 1;
        return self.0;
    }
}
