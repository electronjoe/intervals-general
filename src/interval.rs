use crate::bound_pair::BoundPair;
use std::cmp::Ordering;

/// Interval enum capable of general interval representation
///
/// Where applicable, using lower bound `a` and upper bound `b`.  An Interval taxonomy was pulled from [proofwiki](https://proofwiki.org/wiki/Definition:Real_Interval_Types).
///
/// * Closed -> `[a, b]`
/// * Open -> `(a,b)`
/// * LeftHalfOpen -> `(a, b]`
/// * RightHalfOpen -> `[a, b)`
/// * UnboundedClosedRight -> `(-inf, a]`
/// * UnboundedOpenRight -> `(-inf, a)`
/// * UnboundedClosedLeft -> `[a, inf)`
/// * UnboundedOpenLeft -> `(a, inf)`
/// * Singeleton -> `[a]`
/// * Unbounded -> `(-inf, inf)`
/// * Empty
///
/// # Examples
///
/// ```
/// use intervals_general::bound_pair::BoundPair;
/// use intervals_general::interval::Interval;
/// # fn main() -> std::result::Result<(), String> {
/// let bounds = BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?;
/// let right_half_open = Interval::RightHalfOpen { bound_pair: bounds }; // [1.0, 2.0)
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interval<T> {
    Closed { bound_pair: BoundPair<T> },
    Open { bound_pair: BoundPair<T> },
    LeftHalfOpen { bound_pair: BoundPair<T> },
    RightHalfOpen { bound_pair: BoundPair<T> },
    UnboundedClosedRight { right: T },
    UnboundedOpenRight { right: T },
    UnboundedClosedLeft { left: T },
    UnboundedOpenLeft { left: T },
    Singleton { at: T },
    Unbounded,
    Empty,
}

// Internally used to simplify matching functions on Intervals
enum Bound<T> {
    None,
    Unbounded,
    Open(T),
    Closed(T),
}

impl<T> Interval<T>
where
    T: Copy,
    T: std::cmp::PartialOrd,
    T: std::ops::Sub,
{
    /// Verify whether self contains the specified interval
    ///
    /// Interval I1.contains(I2) if and only if:
    ///
    /// * The left bound of I1 is bounded and less than or equal to the left
    ///   bound of I2 OR
    /// * the left bound of I1 is unbounded and the left bound of I2 is
    ///   unbounded
    ///
    /// AND
    ///
    /// * The right bound of I1 is bounded and greater than or equal to the
    ///   right bound of I2 OR
    /// * The right bound of I1 isunbounded and the left bound of I2 is
    ///   unbounded
    ///
    /// Additionally:
    ///
    /// * The Empty interval does not contain the Empty interval
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// use intervals_general::interval::Interval;
    /// # fn main() -> std::result::Result<(), String> {
    /// let right_half_open = Interval::RightHalfOpen {
    ///     bound_pair: BoundPair::new(1.0, 5.0).ok_or("invalid BoundPair")?,
    /// };
    /// let contained_interval = Interval::Open {
    ///     bound_pair: BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?,
    /// };
    /// let non_contained_interval = Interval::Closed {
    ///     bound_pair: BoundPair::new(4.0, 5.0).ok_or("invalid BoundPair")?,
    /// };
    /// assert_eq!(right_half_open.contains(&contained_interval), true);
    /// assert_eq!(right_half_open.contains(&non_contained_interval), false);
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains(&self, other: &Interval<T>) -> bool {
        let self_right_bound = self.to_right_bound();
        let other_right_bound = other.to_right_bound();
        let self_left_bound = self.to_left_bound();
        let other_left_bound = other.to_left_bound();

        let left_contained = match (self_left_bound, other_left_bound) {
            // The Empty Interval contains no other Intervals (even Empty)
            (Bound::None, _) => false,
            // The Empty interval is contained in all non-Empty Intervals
            (_, Bound::None) => true,
            // If self left interval is unbounded, it will contain any other left bound
            (Bound::Unbounded, _) => true,
            // Given self left interval is not unbounded and right is unbounded, self cannot contain
            // other
            (_, Bound::Unbounded) => false,
            (Bound::Closed(ref self_val), Bound::Closed(ref other_val))
            | (Bound::Closed(ref self_val), Bound::Open(ref other_val))
            | (Bound::Open(ref self_val), Bound::Open(ref other_val)) => {
                if self_val <= other_val {
                    true
                } else {
                    false
                }
            }
            (Bound::Open(ref self_val), Bound::Closed(ref other_val)) => {
                if self_val < other_val {
                    true
                } else {
                    false
                }
            }
        };

        let right_contained = match (self_right_bound, other_right_bound) {
            // The Empty interval does not contain the Empty interval
            (Bound::None, _) => false,
            (_, Bound::None) => false,
            // If self left interval is unbounded, it will contain any other left bound
            (Bound::Unbounded, _) => true,
            // Given self left interval is not unbounded and right is unbounded, self cannot contain
            // other
            (_, Bound::Unbounded) => false,
            (Bound::Closed(ref self_val), Bound::Closed(ref other_val))
            | (Bound::Closed(ref self_val), Bound::Open(ref other_val))
            | (Bound::Open(ref self_val), Bound::Open(ref other_val)) => {
                if self_val >= other_val {
                    true
                } else {
                    false
                }
            }
            (Bound::Open(ref self_val), Bound::Closed(ref other_val)) => {
                if self_val > other_val {
                    true
                } else {
                    false
                }
            }
        };

        left_contained && right_contained
    }

    /// Intersect an with the specified Interval
    ///
    /// Take the intersection of self with the specified Interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// use intervals_general::interval::Interval;
    /// 
    /// # fn main() -> std::result::Result<(), String> {
    /// let i1 = Interval::RightHalfOpen {
    ///     bound_pair: BoundPair::new(1, 5).ok_or("invalid BoundPair")?,
    /// };
    /// let i2 = Interval::Open {
    ///     bound_pair: BoundPair::new(-1, 2).ok_or("invalid BoundPair")?,
    /// };
    /// 
    /// assert_eq!(
    ///     i1.intersect(&i2),
    ///     Interval::RightHalfOpen {
    ///         bound_pair: BoundPair::new(1, 2).ok_or("invalid BoundPair")?
    ///     }
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn intersect(&self, other: &Interval<T>) -> Interval<T> {
        let left_cmp_partial = self.left_partial_cmp(&other);
        let right_cmp_partial = self.right_partial_cmp(&other);
        if left_cmp_partial.is_none() || right_cmp_partial.is_none() {
            return Interval::Empty;
        }

        let left_bound = if left_cmp_partial != Some(Ordering::Less) {
            self.to_left_bound()
        } else {
            other.to_left_bound()
        };
        let right_bound = if right_cmp_partial != Some(Ordering::Greater) {
            self.to_right_bound()
        } else {
            other.to_right_bound()
        };

        match (left_bound, right_bound) {
            (Bound::None, _) => Interval::Empty,
            (_, Bound::None) => Interval::Empty,
            (Bound::Closed(left), Bound::Closed(right)) => {
                if left > right {
                    Interval::Empty
                } else {
                    Interval::Closed {
                        bound_pair: BoundPair { left, right },
                    }
                }
            }
            (Bound::Open(left), Bound::Open(right)) => {
                if left >= right {
                    Interval::Empty
                } else {
                    Interval::Open {
                        bound_pair: BoundPair { left, right },
                    }
                }
            }
            (Bound::Open(left), Bound::Closed(right)) => {
                if left >= right {
                    Interval::Empty
                } else {
                    Interval::LeftHalfOpen {
                        bound_pair: BoundPair { left, right },
                    }
                }
            }
            (Bound::Closed(left), Bound::Open(right)) => {
                if left >= right {
                    Interval::Empty
                } else {
                    Interval::RightHalfOpen {
                        bound_pair: BoundPair { left, right },
                    }
                }
            }
            (Bound::Unbounded, Bound::Closed(right)) => Interval::UnboundedClosedRight { right },
            (Bound::Unbounded, Bound::Open(right)) => Interval::UnboundedOpenRight { right },
            (Bound::Closed(left), Bound::Unbounded) => Interval::UnboundedClosedLeft { left },
            (Bound::Open(left), Bound::Unbounded) => Interval::UnboundedOpenLeft { left },
            (Bound::Unbounded, Bound::Unbounded) => Interval::Unbounded,
        }
    }

    fn to_left_bound(&self) -> Bound<T> {
        match self {
            Interval::Empty => Bound::None,
            Interval::Singleton { ref at } => Bound::Closed(*at),
            // The cases where left bound of self is open -inf
            Interval::Unbounded
            | Interval::UnboundedClosedRight { .. }
            | Interval::UnboundedOpenRight { .. } => Bound::Unbounded,
            // The cases where left bound of self is Closed and Bounded
            Interval::Closed {
                bound_pair: BoundPair { ref left, .. },
            }
            | Interval::RightHalfOpen {
                bound_pair: BoundPair { ref left, .. },
            }
            | Interval::UnboundedClosedLeft { ref left, .. } => Bound::Closed(*left),
            // The cases where left bound of self is Open and Bounded
            Interval::Open {
                bound_pair: BoundPair { ref left, .. },
            }
            | Interval::LeftHalfOpen {
                bound_pair: BoundPair { ref left, .. },
            }
            | Interval::UnboundedOpenLeft { ref left, .. } => Bound::Open(*left),
        }
    }

    fn to_right_bound(&self) -> Bound<T> {
        match self {
            Interval::Empty => Bound::None,
            Interval::Singleton { ref at } => Bound::Closed(*at),
            // The cases where right bound of self is open +inf
            Interval::Unbounded
            | Interval::UnboundedClosedLeft { .. }
            | Interval::UnboundedOpenLeft { .. } => Bound::Unbounded,
            // The cases where right bound of self is Closed and Bounded
            Interval::Closed {
                bound_pair: BoundPair { ref right, .. },
            }
            | Interval::LeftHalfOpen {
                bound_pair: BoundPair { ref right, .. },
            }
            | Interval::UnboundedClosedRight { ref right, .. } => Bound::Closed(*right),
            // The cases where right bound of self is Open and Bounded
            Interval::Open {
                bound_pair: BoundPair { ref right, .. },
            }
            | Interval::RightHalfOpen {
                bound_pair: BoundPair { ref right, .. },
            }
            | Interval::UnboundedOpenRight { ref right, .. } => Bound::Open(*right),
        }
    }

    /// The PartialOrd::partial_cmp implementation for left Bounds
    ///
    /// Though Intervals on some generics (e.g. integers) can supply [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html) because they form a [total order](https://en.wikipedia.org/wiki/Total_order),
    /// unfortunately our floating point implementations break such properties.
    /// Therefore the best we can do under some generics is satisfy [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// use intervals_general::interval::Interval;
    /// use std::cmp::Ordering;
    /// 
    /// # fn main() -> std::result::Result<(), String> {
    /// let right_half_open = Interval::RightHalfOpen {
    ///     bound_pair: BoundPair::new(1.0, 5.0).ok_or("invalid BoundPair")?,
    /// };
    /// let contained_interval = Interval::Open {
    ///     bound_pair: BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?,
    /// };
    /// 
    /// assert_eq!(
    ///     contained_interval.left_partial_cmp(&right_half_open),
    ///     Some(Ordering::Greater)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn left_partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        let self_left_bound = self.to_left_bound();
        let other_left_bound = other.to_left_bound();

        match (self_left_bound, other_left_bound) {
            (Bound::None, _) => None,
            (_, Bound::None) => None,
            // Handle all cases in which one left bound is Unbounded
            (Bound::Unbounded, Bound::Unbounded) => Some(Ordering::Equal),
            (Bound::Unbounded, _) => Some(Ordering::Less),
            (_, Bound::Unbounded) => Some(Ordering::Greater),
            // The cases where left bound of self is Closed and Bounded
            (Bound::Closed(self_val), Bound::Closed(other_val)) => {
                if self_val < other_val {
                    Some(Ordering::Less)
                } else if self_val > other_val {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Bound::Closed(self_val), Bound::Open(other_val)) => {
                if self_val <= other_val {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            // The cases where left bound of self is Open and Bounded
            (Bound::Open(self_val), Bound::Closed(other_val)) => {
                if self_val < other_val {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Bound::Open(self_val), Bound::Open(other_val)) => {
                if self_val < other_val {
                    Some(Ordering::Less)
                } else if self_val > other_val {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }

    /// The PartialOrd::partial_cmp implementation for right Bounds
    ///
    /// Though Intervals on some generics (e.g. integers) can supply [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html) because they form a [total order](https://en.wikipedia.org/wiki/Total_order),
    /// unfortunately our floating point implementations break such properties.
    /// Therefore the best we can do under some generics is satisfy [PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html).
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// use intervals_general::interval::Interval;
    /// use std::cmp::Ordering;
    /// 
    /// # fn main() -> std::result::Result<(), String> {
    /// let right_half_open = Interval::RightHalfOpen {
    ///     bound_pair: BoundPair::new(1.0, 5.0).ok_or("invalid BoundPair")?,
    /// };
    /// let contained_interval = Interval::Open {
    ///     bound_pair: BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?,
    /// };
    /// 
    /// assert_eq!(
    ///     contained_interval.right_partial_cmp(&right_half_open),
    ///     Some(Ordering::Less)
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn right_partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        let self_right_bound = self.to_right_bound();
        let other_right_bound = other.to_right_bound();

        match (self_right_bound, other_right_bound) {
            (Bound::None, _) => None,
            (_, Bound::None) => None,
            // Handle all cases in which one right bound is Unbounded
            (Bound::Unbounded, Bound::Unbounded) => Some(Ordering::Equal),
            (Bound::Unbounded, _) => Some(Ordering::Greater),
            (_, Bound::Unbounded) => Some(Ordering::Less),
            // The cases where right bound of self is Closed and Bounded
            (Bound::Closed(self_val), Bound::Closed(other_val)) => {
                if self_val < other_val {
                    Some(Ordering::Less)
                } else if self_val > other_val {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Bound::Closed(self_val), Bound::Open(other_val)) => {
                if self_val < other_val {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            // The cases where right bound of self is Open and Bounded
            (Bound::Open(self_val), Bound::Closed(other_val)) => {
                if self_val <= other_val {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Bound::Open(self_val), Bound::Open(other_val)) => {
                if self_val < other_val {
                    Some(Ordering::Less)
                } else if self_val > other_val {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
        }
    }
}
