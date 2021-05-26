use core::ops::AddAssign;
use core::ops::SubAssign;

pub trait PostIncr {
    /// `x++` in other language.
    ///
    /// ```
    /// use post_incr::PostIncr as _;
    ///
    /// let mut x: i32 = 0;
    /// assert_eq!(x.post_incr(), 0);
    /// assert_eq!(x.post_incr(), 1);
    /// assert_eq!(x.post_incr(), 2);
    /// ```
    fn post_incr(&mut self) -> Self;
}

pub trait PostDecr {
    /// `x--` in other language.
    ///
    /// ```
    /// use post_incr::PostDecr as _;
    ///
    /// let mut x: i32 = 0;
    /// assert_eq!(x.post_decr(), 0);
    /// assert_eq!(x.post_decr(), -1);
    /// assert_eq!(x.post_decr(), -2);
    /// ```
    fn post_decr(&mut self) -> Self;
}

impl<T> PostIncr for T
where
    Self: Copy + AddAssign + num_traits::One,
{
    fn post_incr(&mut self) -> Self {
        let bk = *self;
        *self += Self::one();
        bk
    }
}

impl<T> PostDecr for T
where
    Self: Copy + SubAssign + num_traits::One,
{
    fn post_decr(&mut self) -> Self {
        let bk = *self;
        *self -= Self::one();
        bk
    }
}
