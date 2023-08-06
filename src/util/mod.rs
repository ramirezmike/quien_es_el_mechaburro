pub mod num_ext {
    pub trait RangedWrap<Rhs = Self> {
        type Output;

        fn sub_with_wrap(self, value: Rhs, max: Rhs) -> Self::Output;
        fn add_with_wrap(self, value: Rhs, max: Rhs) -> Self::Output;
    }

    impl RangedWrap for usize {
        type Output = Self;

        fn sub_with_wrap(self, value: Self, max: Self) -> Self::Output {
            if value > self {
                max - (value - self)
            } else {
                self - value
            }
        }

        fn add_with_wrap(self, value: Self, max: Self) -> Self::Output {
            let result = self + value;
            if result < max {
                result
            } else {
                0
            }
        }
    }
    impl RangedWrap for isize {
        type Output = Self;

        fn sub_with_wrap(self, value: Self, max: Self) -> Self::Output {
            let result = self - value;
            if result < 0 {
                max - 1
            } else {
                result
            }
        }

        fn add_with_wrap(self, value: Self, max: Self) -> Self::Output {
            let result = self + value;
            if result < max {
                result
            } else {
                0
            }
        }
    }
}
