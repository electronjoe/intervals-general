use crate::bound_pair::BoundPair;
use itertools::Either;
use std::cmp::Ordering;

#[cfg(not(feature = "serde"))]
mod without_serde {
    use crate::bound_pair::BoundPair;
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
}

#[cfg(feature = "serde")]
mod with_serde {
    use serde::{Deserialize, Serialize};

    use crate::bound_pair::BoundPair;
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
    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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
}

#[cfg(feature = "serde")]
pub use with_serde::Interval;
#[cfg(not(feature = "serde"))]
pub use without_serde::Interval;

// Internally used to simplify matching functions on Intervals
enum Bound<T> {
    None,
    Unbounded,
    Open(T),
    Closed(T),
}

type TwoIntervalIter<T> =
    std::iter::Chain<std::iter::Once<Interval<T>>, std::iter::Once<Interval<T>>>;
type OneIntervalIter<T> = std::iter::Once<Interval<T>>;

impl<T> Interval<T>
where
    T: Copy,
    T: std::cmp::PartialOrd,
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
        let self_right_bound = self.right_bound();
        let other_right_bound = other.right_bound();
        let self_left_bound = self.left_bound();
        let other_left_bound = other.left_bound();

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
            | (Bound::Open(ref self_val), Bound::Open(ref other_val)) => self_val <= other_val,
            (Bound::Open(ref self_val), Bound::Closed(ref other_val)) => self_val < other_val,
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
            | (Bound::Open(ref self_val), Bound::Open(ref other_val)) => self_val >= other_val,
            (Bound::Open(ref self_val), Bound::Closed(ref other_val)) => self_val > other_val,
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
        let left_cmp_partial = self.left_partial_cmp(other);
        let right_cmp_partial = self.right_partial_cmp(other);
        if left_cmp_partial.is_none() || right_cmp_partial.is_none() {
            return Interval::Empty;
        }

        let left_bound = if left_cmp_partial != Some(Ordering::Less) {
            self.left_bound()
        } else {
            other.left_bound()
        };
        let right_bound = if right_cmp_partial != Some(Ordering::Greater) {
            self.right_bound()
        } else {
            other.right_bound()
        };

        match (left_bound, right_bound) {
            (Bound::None, _) => Interval::Empty,
            (_, Bound::None) => Interval::Empty,
            (Bound::Closed(left), Bound::Closed(right)) => {
                if left > right {
                    Interval::Empty
                } else if left == right {
                    Interval::Singleton { at: left }
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

    fn left_bound(&self) -> Bound<T> {
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

    fn right_bound(&self) -> Bound<T> {
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
        let self_left_bound = self.left_bound();
        let other_left_bound = other.left_bound();

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
        let self_right_bound = self.right_bound();
        let other_right_bound = other.right_bound();

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

    /// Compute the width of the interval
    ///
    /// Returns right - left bound, so long as finite, else None
    /// TODO How to handle overflow detection? I do not have access to check_sub
    /// due to generic? Presently for interval widths exceeding the Boundary
    /// type representation, panic occurs in debug mode and wrapping occurs
    /// in production mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// use intervals_general::interval::Interval;
    ///
    /// # fn main() -> std::result::Result<(), String> {
    /// let interval = Interval::RightHalfOpen {
    ///     bound_pair: BoundPair::new(1, 5).ok_or("invalid BoundPair")?,
    /// };
    ///
    /// let width: i32 = interval.width().ok_or("width was None")?;
    /// assert_eq!(width, 4);
    /// # Ok(())
    /// # }
    /// ```
    pub fn width(&self) -> Option<<T as std::ops::Sub>::Output>
    where
        T: std::ops::Sub,
    {
        let self_left_bound = self.left_bound();
        let self_right_bound = self.right_bound();

        match (self_left_bound, self_right_bound) {
            (Bound::None, _) => None,
            (_, Bound::None) => None,
            (Bound::Unbounded, _) => None,
            (_, Bound::Unbounded) => None,
            (Bound::Closed(left), Bound::Closed(right)) => Some(right - left),
            (Bound::Closed(left), Bound::Open(right)) => Some(right - left),
            (Bound::Open(left), Bound::Closed(right)) => Some(right - left),
            (Bound::Open(left), Bound::Open(right)) => Some(right - left),
        }
    }

    /// Take the complement of the Interval, return one or two Intervals
    ///
    /// The return value is iterable and contains exclusively one or two
    /// Intervals, depending upon result.
    ///
    /// # Example
    ///
    /// ```
    /// use intervals_general::bound_pair::BoundPair;
    /// use intervals_general::interval::Interval;
    ///
    /// # fn main() -> std::result::Result<(), String> {
    /// let mut result_it =
    ///     Interval::Closed {
    ///         bound_pair: BoundPair::new(1, 5).ok_or("invalid BoundPair")?,
    ///     }
    ///     .complement();
    ///
    /// assert_eq!(
    ///     result_it.next(),
    ///     Some(Interval::UnboundedOpenRight { right: 1 })
    /// );
    /// assert_eq!(
    ///     result_it.next(),
    ///     Some(Interval::UnboundedOpenLeft{ left: 5 })
    /// );
    /// assert_eq!(
    ///     result_it.next(),
    ///     None
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn complement(&self) -> itertools::Either<OneIntervalIter<T>, TwoIntervalIter<T>> {
        match self {
            Interval::Closed { bound_pair } => {
                let BoundPair { left, right } = *bound_pair;
                Either::Right(
                    std::iter::once(Interval::UnboundedOpenRight { right: left })
                        .chain(std::iter::once(Interval::UnboundedOpenLeft { left: right })),
                )
            }
            Interval::Open { bound_pair } => {
                let BoundPair { left, right } = *bound_pair;
                Either::Right(
                    std::iter::once(Interval::UnboundedClosedRight { right: left }).chain(
                        std::iter::once(Interval::UnboundedClosedLeft { left: right }),
                    ),
                )
            }
            Interval::LeftHalfOpen { bound_pair } => {
                let BoundPair { left, right } = *bound_pair;
                Either::Right(
                    std::iter::once(Interval::UnboundedClosedRight { right: left })
                        .chain(std::iter::once(Interval::UnboundedOpenLeft { left: right })),
                )
            }
            Interval::RightHalfOpen { bound_pair } => {
                let BoundPair { left, right } = *bound_pair;
                Either::Right(
                    std::iter::once(Interval::UnboundedOpenRight { right: left }).chain(
                        std::iter::once(Interval::UnboundedClosedLeft { left: right }),
                    ),
                )
            }
            Interval::UnboundedClosedRight { right } => {
                Either::Left(std::iter::once(Interval::UnboundedOpenLeft {
                    left: *right,
                }))
            }
            Interval::UnboundedOpenRight { right } => {
                Either::Left(std::iter::once(Interval::UnboundedClosedLeft {
                    left: *right,
                }))
            }
            Interval::UnboundedClosedLeft { left } => {
                Either::Left(std::iter::once(Interval::UnboundedOpenRight {
                    right: *left,
                }))
            }
            Interval::UnboundedOpenLeft { left } => {
                Either::Left(std::iter::once(Interval::UnboundedClosedRight {
                    right: *left,
                }))
            }
            Interval::Singleton { at } => Either::Right(
                std::iter::once(Interval::UnboundedOpenRight { right: *at })
                    .chain(std::iter::once(Interval::UnboundedOpenLeft { left: *at })),
            ),
            Interval::Unbounded => Either::Left(std::iter::once(Interval::Empty)),
            Interval::Empty => Either::Left(std::iter::once(Interval::Unbounded)),
        }
    }
}

/// Implement the Display trait for Intervals
///
/// Here I uses [Wirth Interval Notation](https://proofwiki.org/wiki/Mathematician:Niklaus_Emil_Wirth).
///
/// # Examples
///
/// ```
/// use intervals_general::bound_pair::BoundPair;
/// use intervals_general::interval::Interval;
///
/// # fn main() -> std::result::Result<(), String> {
/// let bp = BoundPair::new(1, 5).ok_or("invalid BoundPair")?;
///
/// assert_eq!(format!("{}", Interval::Closed { bound_pair: bp }), "[1..5]");
/// assert_eq!(
///     format!("{}", Interval::UnboundedOpenRight { right: 5 }),
///     "(←..5)"
/// );
/// # Ok(())
/// # }
/// ```
impl<T> std::fmt::Display for Interval<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Interval::Closed {
                bound_pair:
                    BoundPair {
                        ref left,
                        ref right,
                    },
            } => write!(f, "[{:?}..{:?}]", left, right),
            Interval::Open {
                bound_pair:
                    BoundPair {
                        ref left,
                        ref right,
                    },
            } => write!(f, "({:?}..{:?})", left, right),
            Interval::LeftHalfOpen {
                bound_pair:
                    BoundPair {
                        ref left,
                        ref right,
                    },
            } => write!(f, "({:?}..{:?}]", left, right),
            Interval::RightHalfOpen {
                bound_pair:
                    BoundPair {
                        ref left,
                        ref right,
                    },
            } => write!(f, "[{:?}..{:?})", left, right),
            Interval::UnboundedClosedRight { ref right } => write!(f, "(←..{:?}]", right),
            Interval::UnboundedOpenRight { ref right } => write!(f, "(←..{:?})", right),
            Interval::UnboundedClosedLeft { ref left } => write!(f, "[{:?}..→)", left),
            Interval::UnboundedOpenLeft { ref left } => write!(f, "({:?}..→)", left),
            Interval::Singleton { ref at } => write!(f, "[{:?}]", at),
            Interval::Unbounded => write!(f, "(←..→)"),
            Interval::Empty => write!(f, "Empty"),
        }
    }
}

#[cfg(test)]
mod bound_tests {
    use super::*;

    #[test]
    fn test_left_bound() {
        // Test bounded intervals
        let bp = BoundPair::new(1, 5).unwrap();

        // Closed interval should have closed left bound
        assert!(matches!(
            Interval::Closed { bound_pair: bp }.left_bound(),
            Bound::Closed(1)
        ));

        // Open interval should have open left bound
        assert!(matches!(
            Interval::Open { bound_pair: bp }.left_bound(),
            Bound::Open(1)
        ));

        // Test unbounded intervals
        assert!(matches!(
            Interval::Unbounded::<i32>.left_bound(),
            Bound::Unbounded
        ));

        // Test empty interval
        assert!(matches!(Interval::Empty::<i32>.left_bound(), Bound::None));

        // Test singleton
        assert!(matches!(
            Interval::Singleton { at: 3 }.left_bound(),
            Bound::Closed(3)
        ));

        // Test half-open intervals
        assert!(matches!(
            Interval::LeftHalfOpen { bound_pair: bp }.left_bound(),
            Bound::Open(1)
        ));
        assert!(matches!(
            Interval::RightHalfOpen { bound_pair: bp }.left_bound(),
            Bound::Closed(1)
        ));
    }

    #[test]
    fn test_right_bound() {
        let bp = BoundPair::new(1, 5).unwrap();

        // Test bounded intervals
        assert!(matches!(
            Interval::Closed { bound_pair: bp }.right_bound(),
            Bound::Closed(5)
        ));
        assert!(matches!(
            Interval::Open { bound_pair: bp }.right_bound(),
            Bound::Open(5)
        ));

        // Test special cases
        assert!(matches!(
            Interval::Unbounded::<i32>.right_bound(),
            Bound::Unbounded
        ));
        assert!(matches!(Interval::Empty::<i32>.right_bound(), Bound::None));
        assert!(matches!(
            Interval::Singleton { at: 3 }.right_bound(),
            Bound::Closed(3)
        ));

        // Test unbounded variants
        assert!(matches!(
            Interval::UnboundedClosedLeft { left: 1 }.right_bound(),
            Bound::Unbounded
        ));
        assert!(matches!(
            Interval::UnboundedOpenLeft { left: 1 }.right_bound(),
            Bound::Unbounded
        ));

        // Test half-open intervals
        assert!(matches!(
            Interval::LeftHalfOpen { bound_pair: bp }.right_bound(),
            Bound::Closed(5)
        ));
        assert!(matches!(
            Interval::RightHalfOpen { bound_pair: bp }.right_bound(),
            Bound::Open(5)
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::bound_pair::BoundPair;
    use crate::interval::Interval;
    use itertools::Either;
    use quickcheck::Arbitrary;
    use quickcheck::Gen;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    impl<T> Arbitrary for Interval<T>
    where
        T: Arbitrary + Copy + Clone + PartialOrd + Send + 'static,
    {
        fn arbitrary(g: &mut Gen) -> Interval<T> {
            const VARIANT_COUNT: usize = 12;
            let variant_idx = g.size() % VARIANT_COUNT;

            match variant_idx {
                0 => {
                    let bound_pair = loop {
                        let left = T::arbitrary(g);
                        let right = T::arbitrary(g);
                        if let Some(bp) = BoundPair::new(left, right) {
                            break bp;
                        }
                    };
                    Interval::Closed { bound_pair }
                }
                1 => {
                    let bound_pair = loop {
                        let left = T::arbitrary(g);
                        let right = T::arbitrary(g);
                        if let Some(bp) = BoundPair::new(left, right) {
                            break bp;
                        }
                    };
                    Interval::Open { bound_pair }
                }
                2 => {
                    let bound_pair = loop {
                        let left = T::arbitrary(g);
                        let right = T::arbitrary(g);
                        if let Some(bp) = BoundPair::new(left, right) {
                            break bp;
                        }
                    };
                    Interval::LeftHalfOpen { bound_pair }
                }
                3 => {
                    let bound_pair = loop {
                        let left = T::arbitrary(g);
                        let right = T::arbitrary(g);
                        if let Some(bp) = BoundPair::new(left, right) {
                            break bp;
                        }
                    };
                    Interval::LeftHalfOpen { bound_pair }
                }
                4 => {
                    let bound_pair = loop {
                        let left = T::arbitrary(g);
                        let right = T::arbitrary(g);
                        if let Some(bp) = BoundPair::new(left, right) {
                            break bp;
                        }
                    };
                    Interval::RightHalfOpen { bound_pair }
                }
                5 => Interval::UnboundedClosedRight {
                    right: T::arbitrary(g),
                },
                6 => Interval::UnboundedOpenRight {
                    right: T::arbitrary(g),
                },
                7 => Interval::UnboundedClosedLeft {
                    left: T::arbitrary(g),
                },
                8 => Interval::UnboundedOpenLeft {
                    left: T::arbitrary(g),
                },
                9 => Interval::Singleton {
                    at: T::arbitrary(g),
                },
                10 => Interval::Unbounded,
                11 => Interval::Empty,
                _ => unreachable!("variant_idx is always < VARIANT_COUNT"),
            }
        }

        // fn shrink(&self) -> Box<Iterator<Item = Self>> {
        //     match self {
        //         // &Interval::Unbounded => Box::new(Interval::Unbounded),
        //         // &Qqq::Kokoko(ref x) => Box::new(x.shrink().map(|s| Qqq::Kokoko(s))),
        //         _ => quickcheck::empty_shrinker(),
        //     }
        // }
    }

    #[test]
    fn test_bounded_complements() {
        let bp = BoundPair::new(1, 5).unwrap();
        let mut it = Interval::Closed { bound_pair: bp }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedOpenRight { right: 1 }));
        assert_eq!(it.next(), Some(Interval::UnboundedOpenLeft { left: 5 }));
        assert_eq!(it.next(), None);

        it = Interval::Open { bound_pair: bp }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedClosedRight { right: 1 }));
        assert_eq!(it.next(), Some(Interval::UnboundedClosedLeft { left: 5 }));
        assert_eq!(it.next(), None);

        it = Interval::LeftHalfOpen { bound_pair: bp }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedClosedRight { right: 1 }));
        assert_eq!(it.next(), Some(Interval::UnboundedOpenLeft { left: 5 }));
        assert_eq!(it.next(), None);

        it = Interval::RightHalfOpen { bound_pair: bp }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedOpenRight { right: 1 }));
        assert_eq!(it.next(), Some(Interval::UnboundedClosedLeft { left: 5 }));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_unbounded_complements() {
        let mut it = Interval::UnboundedClosedRight { right: 5 }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedOpenLeft { left: 5 }));
        assert_eq!(it.next(), None);

        it = Interval::UnboundedOpenRight { right: 5 }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedClosedLeft { left: 5 }));
        assert_eq!(it.next(), None);

        it = Interval::UnboundedClosedLeft { left: 1 }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedOpenRight { right: 1 }));
        assert_eq!(it.next(), None);

        it = Interval::UnboundedOpenLeft { left: 1 }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedClosedRight { right: 1 }));
        assert_eq!(it.next(), None);

        let mut it = Interval::Singleton { at: 2.0 }.complement();
        assert_eq!(it.next(), Some(Interval::UnboundedOpenRight { right: 2.0 }));
        assert_eq!(it.next(), Some(Interval::UnboundedOpenLeft { left: 2.0 }));
        assert_eq!(it.next(), None);

        it = Interval::Unbounded.complement();
        assert_eq!(it.next(), Some(Interval::Empty));
        assert_eq!(it.next(), None);

        it = Interval::Empty.complement();
        assert_eq!(it.next(), Some(Interval::Unbounded));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn interval_display() {
        let bp = BoundPair::new(1, 5).ok_or("invalid BoundPair").unwrap();

        assert_eq!(format!("{}", Interval::Closed { bound_pair: bp }), "[1..5]");
        assert_eq!(format!("{}", Interval::Open { bound_pair: bp }), "(1..5)");
        assert_eq!(
            format!("{}", Interval::LeftHalfOpen { bound_pair: bp }),
            "(1..5]"
        );
        assert_eq!(
            format!("{}", Interval::RightHalfOpen { bound_pair: bp }),
            "[1..5)"
        );
        assert_eq!(
            format!("{}", Interval::UnboundedClosedRight { right: 5 }),
            "(←..5]"
        );
        assert_eq!(
            format!("{}", Interval::UnboundedOpenRight { right: 5 }),
            "(←..5)"
        );
        assert_eq!(
            format!("{}", Interval::UnboundedClosedLeft { left: 1 }),
            "[1..→)"
        );
        assert_eq!(
            format!("{}", Interval::UnboundedOpenLeft { left: 1 }),
            "(1..→)"
        );
        assert_eq!(format!("{}", Interval::Singleton { at: 3.0 }), "[3.0]");
        assert_eq!(format!("{}", Interval::Unbounded::<u32> {}), "(←..→)");
        assert_eq!(format!("{}", Interval::Empty::<u32> {}), "Empty");
    }

    #[quickcheck]
    fn intersect_strictly_shrinks_u32(l1: u32, l2: u32, r1: u32, r2: u32) -> TestResult {
        if let (Some(bp1), Some(bp2)) = (BoundPair::new(l1, r1), BoundPair::new(l2, r2)) {
            let i1 = Interval::LeftHalfOpen { bound_pair: bp1 };
            let i2 = Interval::LeftHalfOpen { bound_pair: bp2 };
            let intersection = i1.intersect(&i2);
            TestResult::from_bool(
                !(intersection.width() > i1.width() || intersection.width() > i2.width()),
            )
        } else {
            // Discard invalid randomly generated intervals
            TestResult::discard()
        }
    }

    #[quickcheck]
    fn intersect_strictly_shrinks_f32(l1: f32, l2: f32, r1: f32, r2: f32) -> TestResult {
        if let (Some(bp1), Some(bp2)) = (BoundPair::new(l1, r1), BoundPair::new(l2, r2)) {
            let i1 = Interval::LeftHalfOpen { bound_pair: bp1 };
            let i2 = Interval::LeftHalfOpen { bound_pair: bp2 };
            let intersection = i1.intersect(&i2);
            TestResult::from_bool(
                !(intersection.width() > i1.width() || intersection.width() > i2.width()),
            )
        } else {
            // Discard invalid randomly generated intervals
            TestResult::discard()
        }
    }

    #[quickcheck]
    fn complement_symmetric_u32(i: Interval<u32>) -> TestResult {
        let double_complement = match i.complement() {
            Either::Left(mut interval) => interval.next().unwrap().complement().next().unwrap(),
            Either::Right(mut intervals) => {
                let [i1, i2] = [intervals.next().unwrap(), intervals.next().unwrap()];
                i1.complement()
                    .next()
                    .unwrap()
                    .intersect(&i2.complement().next().unwrap())
            }
        };

        TestResult::from_bool(double_complement == i)
    }

    #[test]
    fn test_intersection_edge_cases() {
        // Test intersection resulting in singleton
        let left_interval = Interval::Closed {
            bound_pair: BoundPair::new(0, 5).unwrap(),
        };
        let right_interval = Interval::Closed {
            bound_pair: BoundPair::new(5, 10).unwrap(),
        };

        // Intersection at single point should yield singleton
        assert_eq!(
            left_interval.intersect(&right_interval),
            Interval::Singleton { at: 5 }
        );

        // Test open interval edge cases
        let left_open = Interval::Open {
            bound_pair: BoundPair::new(0, 5).unwrap(),
        };
        let right_open = Interval::Open {
            bound_pair: BoundPair::new(5, 10).unwrap(),
        };

        // Open intervals touching should yield empty
        assert_eq!(left_open.intersect(&right_open), Interval::Empty);
    }

    #[test]
    fn test_empty_interval_intersections() {
        let normal_interval = Interval::Closed {
            bound_pair: BoundPair::new(0, 5).unwrap(),
        };
        let empty = Interval::Empty;

        // Empty interval intersected with any interval should yield empty
        assert_eq!(empty.intersect(&normal_interval), Interval::Empty);
        assert_eq!(normal_interval.intersect(&empty), Interval::Empty);
        assert_eq!(empty.intersect(&empty), Interval::Empty);
    }

    #[test]
    fn test_basic_contains() {
        let outer = Interval::Closed {
            bound_pair: BoundPair::new(0, 10).unwrap(),
        };
        let inner = Interval::Closed {
            bound_pair: BoundPair::new(2, 8).unwrap(),
        };
        assert!(outer.contains(&inner));
        assert!(!inner.contains(&outer));
    }

    #[test]
    fn test_empty_interval_contains() {
        let interval = Interval::Closed {
            bound_pair: BoundPair::new(0, 10).unwrap(),
        };
        let empty = Interval::Empty;

        // The empty interval is not contained by any interval
        assert!(!interval.contains(&empty));
        // Empty interval contains nothing, not even itself
        assert!(!empty.contains(&empty));
        assert!(!empty.contains(&interval));
    }

    #[test]
    fn test_unbounded_contains() {
        let unbounded = Interval::Unbounded;
        let finite = Interval::Closed {
            bound_pair: BoundPair::new(0, 10).unwrap(),
        };

        assert!(unbounded.contains(&finite));
        assert!(!finite.contains(&unbounded));
    }

    #[test]
    fn test_mixed_bound_types() {
        let closed = Interval::Closed {
            bound_pair: BoundPair::new(0, 10).unwrap(),
        };
        let open = Interval::Open {
            bound_pair: BoundPair::new(0, 10).unwrap(),
        };

        // Closed interval contains its open counterpart
        assert!(closed.contains(&open));
        // Open interval does not contain its closed counterpart
        assert!(!open.contains(&closed));
    }

    #[test]
    fn test_singleton_contains() {
        let singleton = Interval::Singleton { at: 5 };
        let containing = Interval::Closed {
            bound_pair: BoundPair::new(0, 10).unwrap(),
        };
        let not_containing = Interval::Open {
            bound_pair: BoundPair::new(0, 5).unwrap(),
        };

        assert!(containing.contains(&singleton));
        // Open interval does not contain singleton on its bounds
        assert!(!not_containing.contains(&singleton));
        // Singleton only contains itself
        assert!(singleton.contains(&singleton));
    }

    #[quickcheck]
    fn prop_contains_transitive(a: f64, b: f64, c: f64) -> TestResult {
        if let (Some(bp1), Some(bp2), Some(bp3)) = (
            BoundPair::new(a, b),
            BoundPair::new(b, c),
            BoundPair::new(a, c),
        ) {
            let i1 = Interval::Closed { bound_pair: bp1 };
            let i2 = Interval::Closed { bound_pair: bp2 };
            let i3 = Interval::Closed { bound_pair: bp3 };

            TestResult::from_bool(!(i1.contains(&i2) && i2.contains(&i3)) || i1.contains(&i3))
        } else {
            TestResult::discard()
        }
    }
}
