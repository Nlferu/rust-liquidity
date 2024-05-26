macro_rules! impl_add_assign {
    ($($t:ty)*) => ($(
        impl std::ops::AddAssign for $t {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }
    )*)
}
