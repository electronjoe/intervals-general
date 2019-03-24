/// A BoundPair represents valid left and right Interval bounds
///
/// For Intervals containing finite bounds, the BoundPair construction
/// ensures well-formed left and right bounds prior to Interval enum
/// construction (e.g. left < right).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoundPair<T> {
    left: T,
    right: T,
}

impl<T> BoundPair<T>
where
    T: Copy,
    T: PartialOrd,
{
    /// Create a new Bound Pair with lower and upper bounds. If the bounds
    /// are mal-formed return None.
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// use intervals_general::interval::Interval;
    /// # fn main() -> std::result::Result<(), String> {
    /// let bounds = BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?;
    /// let right_half_open =
    ///   Interval::RightHalfOpen{ bound_pair: bounds };  // [1.0, 2.0)
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Failures
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// assert_eq!(BoundPair::new(2,1), None);
    /// assert_eq!(BoundPair::new(2.0, 2.0), None);
    /// ```
    pub fn new(left: T, right: T) -> Option<BoundPair<T>> {
        if left >= right {
            return None;
        } else {
            return Some(BoundPair { left, right });
        }
    }
}
