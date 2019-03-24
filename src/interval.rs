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
/// let right_half_open =
///   Interval::RightHalfOpen{ bound_pair: bounds };  // [1.0, 2.0)
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
