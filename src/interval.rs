use crate::bound_pair::BoundPair;
use itertools::Either;
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
        let self_left_bound = self.to_left_bound();
        let self_right_bound = self.to_right_bound();

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
            Interval::Closed { bound_pair: bp } => Either::Right(
                std::iter::once(Interval::UnboundedOpenRight { right: bp.left }).chain(
                    std::iter::once(Interval::UnboundedOpenLeft { left: bp.right }),
                ),
            ),
            Interval::Open { bound_pair: bp } => Either::Right(
                std::iter::once(Interval::UnboundedClosedRight { right: bp.left }).chain(
                    std::iter::once(Interval::UnboundedClosedLeft { left: bp.right }),
                ),
            ),
            Interval::LeftHalfOpen { bound_pair: bp } => Either::Right(
                std::iter::once(Interval::UnboundedClosedRight { right: bp.left }).chain(
                    std::iter::once(Interval::UnboundedOpenLeft { left: bp.right }),
                ),
            ),
            Interval::RightHalfOpen { bound_pair: bp } => Either::Right(
                std::iter::once(Interval::UnboundedOpenRight { right: bp.left }).chain(
                    std::iter::once(Interval::UnboundedClosedLeft { left: bp.right }),
                ),
            ),
            Interval::UnboundedClosedRight { right: r } => {
                Either::Left(std::iter::once(Interval::UnboundedOpenLeft { left: *r }))
            }
            Interval::UnboundedOpenRight { right: r } => {
                Either::Left(std::iter::once(Interval::UnboundedClosedLeft { left: *r }))
            }
            Interval::UnboundedClosedLeft { left: l } => {
                Either::Left(std::iter::once(Interval::UnboundedOpenRight { right: *l }))
            }
            Interval::UnboundedOpenLeft { left: l } => {
                Either::Left(std::iter::once(Interval::UnboundedClosedRight {
                    right: *l,
                }))
            }
            Interval::Singleton { at: a } => Either::Right(
                std::iter::once(Interval::UnboundedOpenRight { right: *a })
                    .chain(std::iter::once(Interval::UnboundedOpenLeft { left: *a })),
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
mod tests {
    use crate::bound_pair::BoundPair;
    use crate::interval::Interval;
    use itertools::Either;
    use quickcheck::Arbitrary;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    impl<T> Arbitrary for Interval<T>
    where
        T: Arbitrary + Copy + Clone + PartialOrd + Send + 'static,
    {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Interval<T> {
            let variant_idx = g.next_u32() % 12;
            match variant_idx {
                0 => {
                    let mut bound_pair = None;
                    while let None = bound_pair {
                        bound_pair =
                            BoundPair::new(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g));
                    }
                    Interval::Closed {
                        bound_pair: bound_pair.unwrap(),
                    }
                }
                1 => {
                    let mut bound_pair = None;
                    while let None = bound_pair {
                        bound_pair =
                            BoundPair::new(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g));
                    }
                    Interval::Open {
                        bound_pair: bound_pair.unwrap(),
                    }
                }
                2 => {
                    let mut bound_pair = None;
                    while let None = bound_pair {
                        bound_pair =
                            BoundPair::new(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g));
                    }
                    Interval::LeftHalfOpen {
                        bound_pair: bound_pair.unwrap(),
                    }
                }
                3 => {
                    let mut bound_pair = None;
                    while let None = bound_pair {
                        bound_pair =
                            BoundPair::new(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g));
                    }
                    Interval::LeftHalfOpen {
                        bound_pair: bound_pair.unwrap(),
                    }
                }
                4 => {
                    let mut bound_pair = None;
                    while let None = bound_pair {
                        bound_pair =
                            BoundPair::new(Arbitrary::arbitrary(g), Arbitrary::arbitrary(g));
                    }
                    Interval::RightHalfOpen {
                        bound_pair: bound_pair.unwrap(),
                    }
                }
                5 => Interval::UnboundedClosedRight {
                    right: Arbitrary::arbitrary(g),
                },
                6 => Interval::UnboundedOpenRight {
                    right: Arbitrary::arbitrary(g),
                },
                7 => Interval::UnboundedClosedLeft {
                    left: Arbitrary::arbitrary(g),
                },
                8 => Interval::UnboundedOpenLeft {
                    left: Arbitrary::arbitrary(g),
                },
                9 => Interval::Singleton {
                    at: Arbitrary::arbitrary(g),
                },
                10 => Interval::Unbounded,
                11 => Interval::Empty,
                _ => panic!(),
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
            if (intersection.width() > i1.width()) | (intersection.width() > i2.width()) {
                TestResult::from_bool(false)
            } else {
                TestResult::from_bool(true)
            }
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
            if (intersection.width() > i1.width()) | (intersection.width() > i2.width()) {
                TestResult::from_bool(false)
            } else {
                TestResult::from_bool(true)
            }
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
                let i1 = intervals.next().unwrap();
                let i2 = intervals.next().unwrap();
                i1.complement()
                    .next()
                    .unwrap()
                    .intersect(&i2.complement().next().unwrap())
            }
        };

        TestResult::from_bool(double_complement == i)
    }
}
