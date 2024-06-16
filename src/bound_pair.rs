#[cfg(not(feature = "serde"))]
mod without_serde {
    /// A BoundPair represents valid left and right Interval bounds
    ///
    /// For Intervals containing finite bounds, the BoundPair construction
    /// ensures well-formed left and right bounds prior to Interval enum
    /// construction (e.g. left < right).
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct BoundPair<T> {
        pub(crate) left: T,
        pub(crate) right: T,
    }
}

#[cfg(feature = "serde")]
mod with_serde {
    use serde::{Deserialize, Serialize};

    /// A BoundPair represents valid left and right Interval bounds
    ///
    /// For Intervals containing finite bounds, the BoundPair construction
    /// ensures well-formed left and right bounds prior to Interval enum
    /// construction (e.g. left < right).
    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    pub struct BoundPair<T> {
        pub(crate) left: T,
        pub(crate) right: T,
    }
}

#[cfg(feature = "serde")]
pub use with_serde::BoundPair;
#[cfg(not(feature = "serde"))]
pub use without_serde::BoundPair;

impl<T> BoundPair<T>
where
    T: Copy,
    T: PartialOrd,
{
    /// Create a new Bound Pair with lower and upper bounds.
    ///
    /// If the bounds are mal-formed, i.e. !(left < right), return None.
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// # fn main() -> std::result::Result<(), String> {
    /// let bounds = BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Failures
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// assert_eq!(BoundPair::new(2, 1), None);
    /// assert_eq!(BoundPair::new(2.0, 2.0), None);
    /// ```
    pub fn new(left: T, right: T) -> Option<BoundPair<T>> {
        if left >= right {
            None
        } else {
            Some(BoundPair { left, right })
        }
    }

    /// Fetch an immutable reference to the left bound
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// # fn main() -> std::result::Result<(), String> {
    /// let bounds = BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?;
    /// assert_eq!(*bounds.left(), 1.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn left(&self) -> &T {
        &self.left
    }

    /// Fetch an immutable reference to the right bound
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// # fn main() -> std::result::Result<(), String> {
    /// let bounds = BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?;
    /// assert_eq!(*bounds.right(), 2.0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn right(&self) -> &T {
        &self.right
    }
}
