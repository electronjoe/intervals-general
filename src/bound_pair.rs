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
        match left.partial_cmp(&right) {
            Some(std::cmp::Ordering::Less) => Some(BoundPair { left, right }),
            _ => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_creation() {
        assert!(BoundPair::new(1, 2).is_some());
        assert!(BoundPair::new(-1.0, 1.0).is_some());
        assert!(BoundPair::new(0u32, 100u32).is_some());
    }

    #[test]
    fn test_invalid_creation() {
        assert!(BoundPair::new(2, 1).is_none());
        assert!(BoundPair::new(1.0, 1.0).is_none());
        assert!(BoundPair::new(100u32, 50u32).is_none());
    }

    #[test]
    fn test_accessors() {
        let bp = BoundPair::new(1.5, 2.5).unwrap();
        assert_eq!(*bp.left(), 1.5);
        assert_eq!(*bp.right(), 2.5);
    }

    #[test]
    fn test_floating_point() {
        assert!(BoundPair::new(f64::NEG_INFINITY, f64::INFINITY).is_some());
        assert!(BoundPair::new(f64::NAN, 1.0).is_none());
        assert!(BoundPair::new(1.0, f64::NAN).is_none());
    }

    #[test]
    fn test_copy() {
        let bp1 = BoundPair::new(1, 2).unwrap();
        let bp2 = bp1;
        assert_eq!(bp1, bp2);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize() {
        let bp = BoundPair::new(1, 2).unwrap();
        let serialized = serde_json::to_string(&bp).unwrap();
        assert_eq!(serialized, r#"{"left":1,"right":2}"#);
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"left":1,"right":2}"#;
        let bp: BoundPair<i32> = serde_json::from_str(json).unwrap();
        assert_eq!(*bp.left(), 1);
        assert_eq!(*bp.right(), 2);
    }

    #[test]
    fn test_roundtrip() {
        let bp1 = BoundPair::new(1.5, 2.5).unwrap();
        let serialized = serde_json::to_string(&bp1).unwrap();
        let bp2: BoundPair<f64> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bp1, bp2);
    }
}
