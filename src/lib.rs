//! This crate enables generalized Interval representation and operations
//!
//! Supporting generic bound data types (e.g. compatible with units-of-measure
//! to enable typechecked physical units) - and supporting all necessary
//! Interval representations for closure of interval operations.  See README.md
//! for detailed design discussion.
//!
//! ## Examples
//!
//! ```
//! use intervals_general::bound_pair::BoundPair;
//! use intervals_general::interval::Interval;
//! # fn main() -> std::result::Result<(), String> {
//! let bounds = BoundPair::new(1.0, 2.0).ok_or("invalid BoundPair")?;
//! let right_half_open = Interval::RightHalfOpen { bound_pair: bounds }; // [1.0, 2.0)
//! # Ok(())
//! # }
//! ```
//!
//! ## Requirements
//!
//! ### Support For
//!
//! 1. Intervals with bound data types provied via generic
//! 1. [Open](https://proofwiki.org/wiki/Definition:Real_Interval_Types#Open_Interval), [closed](https://proofwiki.org/wiki/Definition:Real_Interval_Types#Closed_Interval) and [half-open](https://proofwiki.org/wiki/Definition:Real_Interval/Half-Open) Intervals
//! 1. Type-enforced representation of the [empty](https://proofwiki.org/wiki/Definition:Real_Interval/Empty) Interval
//! 1. Type-enforced representation of [unbounded](https://proofwiki.org/wiki/Definition:Real_Interval_Types#Unbounded_Intervals) Intervals
//!
//! #### Implementation Constraints
//!
//! 1. no_std support
//! 1. No use of of panic, assert
//! 1. Minimize error handling by design
//! 1. Make the library hard to use incorrectly
pub mod bound_pair;
pub mod interval;
