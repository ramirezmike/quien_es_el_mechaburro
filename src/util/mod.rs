pub mod num_ext {
    pub trait RangedWrap {
        fn circular_increment(self, lower_bound: Self, upper_bound: Self) -> Self;
        fn circular_decrement(self, lower_bound: Self, upper_bound: Self) -> Self;
    }

    impl RangedWrap for usize {
        fn circular_increment(self, lower_bound: Self, upper_bound: Self) -> Self {
            let range = upper_bound - lower_bound + 1;
            ((self - lower_bound + 1) % range) + lower_bound
        }

        fn circular_decrement(self, lower_bound: Self, upper_bound: Self) -> Self {
            let range = upper_bound - lower_bound + 1;
            ((self - lower_bound + range - 1) % range) + lower_bound
        }
    }

    // TODO: combine these
    impl RangedWrap for isize {
        fn circular_increment(self, lower_bound: Self, upper_bound: Self) -> Self {
            let range = upper_bound - lower_bound + 1;
            ((self - lower_bound + 1) % range) + lower_bound
        }

        fn circular_decrement(self, lower_bound: Self, upper_bound: Self) -> Self {
            let range = upper_bound - lower_bound + 1;
            ((self - lower_bound + range - 1) % range) + lower_bound
        }
    }
}
