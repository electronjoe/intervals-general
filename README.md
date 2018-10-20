# uom-intervals Crate RFC

* Start Date Oct 13, 2018
* Work in Progress
* Scott Moeller

## Summary

Addition of a new Crate named uom-intervals which supports rigorous interval definitions, interval collections and common interval operations, all while enforcing units of measure.

## Motivation

In working to write a simulation tool with support for e.g. step function representations that enforce units of measure - I found myself looking for an interval representation that would accept [uom](https://crates.io/crates/uom) types and was capable of sufficient expressivity so as to be closed under common Interval operations of {Union, Intersection, Complement}.  Not finding a crate that met those criteria, I looked at other language implementations and began writing a proposal for a child-crate for [uom](https://crates.io/crates/uom).

### Requirements

The Requirements for the library are then:

1. Support for intervals that enforce units of measure
1. Support for [open](https://proofwiki.org/wiki/Definition:Real_Interval_Types#Open_Interval), [closed](https://proofwiki.org/wiki/Definition:Real_Interval_Types#Closed_Interval) and [half-open](https://proofwiki.org/wiki/Definition:Real_Interval/Half-Open) Intervals
1. Support for the type-enforced representation of the [empty](https://proofwiki.org/wiki/Definition:Real_Interval/Empty) Interval
1. Support for the type-enforced representation of [unbounded](https://proofwiki.org/wiki/Definition:Real_Interval_Types#Unbounded_Intervals) Intervals

Also nice to have

1. no_std support
1. No use of of panic, assert
1. Minimize error handling by design

### Requirement Reasoning

As a motivating case consider the use of this Intervals library to build a step function library for physics simulations.  Lets omit for now the design decision to use Interval representations to back the step function library (versus say "change point" annotations).  Commonly in such a setting, you may want:

* Representation of a signal whose domain covers (-inf, to +inf) in some units
* The collection of Step Function Intervals forming the Domain is continuous

In this space, operating under exclusively [Closed](https://proofwiki.org/wiki/Definition:Real_Interval/Closed) Intervals is a non-starter (neither can you include the [Unbounded Intervals](https://proofwiki.org/wiki/Definition:Real_Interval_Types#Unbounded_Intervals) nor can you define a collection of Intervals whose Domain is continuous in e.g. Real Numbers but whose Intervals do not overlap).  Operating under exclusively open sets is also problematic (one cannot define a collection of Intervals whose Domain is continuous in e.g. Real Numbers but are each open).  If you stick with exclusively Left-Half-Open or Right-Half-Open Intervals, you can define collections of Intervals that are continuous and do not overlap - but you now cannot cleanly express Unbounded Intervals of both flavors (e.g. Left-Half-Open only Intervals prevents you from using a +inf bound, forcing the author to select some "sufficiently large" offset for the right closed bound as to be approximately +inf for their use case).  Finally, note that if a mixture of Left-Half-Open and Right-Half-Open bound types are supported, then under Intersection the closure of Interval tyupes that must be supported becomes {Open, Closed, Left-Half-Open, Right-Half-Open, Left-Half-Open-Unbounded, Right-Half-Open-Unbounded, [Singleton](https://proofwiki.org/wiki/Definition:Real_Interval/Singleton), Empty}.  Omitting any of these flavors will require introduction of error handling or exceptions - which I strongly desire to avoid.  Therefore the desire to support the full set of Interval types (see [proofwiki:Real Interval Types](https://proofwiki.org/wiki/Definition:Real_Interval_Types)).

## Detailed Design

### Terminology Selection

* Propose the use of **Interval** instead of *Range*
  * Gives a clearer intent of the mathematical construct
  * Avoids confusion with the many range concepts that occur in programming languages
* Propose the use of **Bound** instead of *Endpoint* or *Limit*
  * Though at this time I see no advantage either way
* Propose the use of **Left Bound** and **Right Bound**
  * This seems to be more Interval-centric than e.g. min, max

### Interval Operations Supported

1. Union
1. Intersection
1. Complement
1. Left Partial Compare
1. Right Partial Compare

From these, many useful utilities can be derived.  Several examples follow:

1. Element containment: is [1, 1] contained in [-1, 5]? -> Union of sets equals [1, 1]? -> true
1. Interval containment: is (1, 2] contained in [-1, 5]? -> Union of sets equals (1,2]? -> true
1. Sorting of intervals by (left|right) Bound

### Representation of Interval Bound Types

Intervals are represented by an Enum, representing all combinations of open, closed, and half-open intervals and their ray variants (one bound at +/- inf).  See the alternative discussion [below](#alternative-representation-of-interval-bound-types) for evaluation of a representation in which Intervals contain a Left and Right Bound Enum instead.

Nomenclature below is pulled from [proofwiki:Real Interval Types](https://proofwiki.org/wiki/Definition:Real_Interval_Types).

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interval<T> {
    ClosedBounded { left: T, right: T },         # [a, b]
    OpenBounded { left: T, right: T },           # (a, b)
    LeftHalfOpenBounded { left: T, right: T },   # (a, b]
    RightHalfOpenBounded { left: T, right: T },  # [a, b)
    ClosedBoundedRight { right: T },             # (-inf, a]
    OpenBoundedRight { right: T },               # (-inf, a)
    ClosedBoundedLeft { left: T },               # [a, inf)
    OpenBoundedLeft { left: T },                 # (a, inf)
    Unbounded,                                   # (-inf, inf)
    Empty,                                       # Empty Interval
}
```

### Handling Bounds at Interval Creation

See the discussion [below](#unresolved-handling-bounds-at-interval-creation) on alternatives for error checking of Interval Bounds during creation.

### Traits Required by Bound Data Type

* T: std::cmp::PartialOrd

This is required in order to support comparison between interval Bounds, a necessity for many interval operations.

## Drawbacks

### Computational Complexity

The computation complexity of interval operations is increased due to the dynamic runtime representation and support for operations across interval types (i.e. including support for type-enforced open, closed, and half-open intervals, as well as type-enforced representation for open infinite bounds).  One C++ library offers both runtime dynamic OR static Interval support (see Boost Interval Container Library [below](#existing-libraries-from-other-languages).  It might be educational to prototype interval operations in order to compare performance against a simple interval representation that is e.g. left-half-open for all intervals and does not support general intervals.

## Alternatives

### Existing Crates

There are existig crates in this space, and it would be improper to create a new crate if contribution to the existing ecosystem was a reasonable path forward.  So here I'll enumerate the crates I have found - and why I believe they are not aligned with the goals listed in [Motivation](#motivation).  I'll also call out functionlity offered by these crates that may be appealing for uom-intervals.  I'll reach out to the listed crate owners and gather their feedback before moving forward on uom-intervals.

[Honest Intervals](https://crates.io/crates/honestintervals)

[Interval](https://crates.io/crates/interval)

[Interval Arithmetic Library](https://crates.io/crates/intervallum) - Support for exclusively Closed Bounded intervals, Singleton and Empty (via IsSingleton, IsEmpty).  Supports primitive types only for Bounds (no uom support). Approximates unbounded intervals with bounds at sentinal values pulled from the primitive Bound extremum (e.g. -128 for i8 -> [inf).

### Existing Libraries from Other Languages

[Haskell intervals package](http://hackage.haskell.org/package/intervals) - support for Open Bounded and Unbounded intervals only.  Good collection of operations on Intervals including (member, infimum, supremum, width, bisect, intersect, hull, distance, inflate, deflate, scale, comparison, and various utility Interval constructors).

[Haskell IntervalMap package](http://hackage.haskell.org/package/IntervalMap) - Set and Map implementations that support Intervals as keys and functions for efficiently fetching a subset of all intervals containing a point, intersecting an interval, and more.

[Boost Interval Container Library - ICL](https://www.boost.org/doc/libs/1_60_0/libs/icl/doc/html/boost_icl/examples/interval.html) - Support for static compile-time types right_open_interval, left_open_interval, closed_interval, open_interval (all bounded).  Also supports dynamic runtime discrete_interval and continuous_interval which can be built with left_open or right_open specifications.  The offering of static compile-time variants is interesting, as it offers a reduced computation burden if the consumer can stick to a single Interval type and can ensure consistent interval type is used in applications wanting only e.g. left_open intervals.  The continuous interval supports strings and dates, an interesting concept.  Also implemented are interval sets and interval maps.

### Alternative Representation of Interval Bound Types

I have explored multiple options for the type-enforced representations desired for the library, including:

1. An Enum Bound without Left / Right binding and Capable of representing Open-Infinite bounds (e.g. Closed\<data type\>, OpenFinite\<data type\>, OpenInfinite)
1. An Enum Bound without Left / Right binding and incapable of representing Open-Infinite bounds (e.g. Closed\<data type\>, Open\<data type\>) where *data type* supports type-enforced infinity representation
1. An Enum Bound with Left/Right binding (e.g. LeftOpenFinite\<data type\>, LeftOpenInfinite, LeftClosed, RightOpenFinite, RightOpenInfinite, RightClosed)

```rust
enum LeftBound<T> {
    Unbounded,
    OpenBounded(T),
    ClosedBounded(T),
}

enum RightBound<T> {
    Unbounded,
    OpenBounded(T),
    ClosedBounded(T),
}
```

## Unresolved Questions

### Unresolved Rounding Mode a Concern

Depending upon the arithmetic being applied to intervals, rounding error could result in a set of intervals mutating beyond expectation. I have convinced myself that the rounding issues are no more severe than you would experience from general floating point use representations (that is, no Interval specific thorns appear to be raised) - specifically even under scaling (e.g. *2.5 [1, 2) = [2.5, 5.0)) a collection of intervals which were continuous in the Real Domain prior to scaling - remain continuous in the Real Domain after scaling.  Therefore rounding errors cannot cause Continuous collections of Intervals to becoe discontinuous for some Domain (ditto for offset modifications to Intervals, e.g. +2.5 [1, 2) = [3.5, 4.5)).  If however the width of an Interval is a property that is intended to be relied upon (e.g. [Interval Arithmetic](http://mathworld.wolfram.com/IntervalArithmetic.html)) - then rounding mode is an important consideration as it can ruin expected properties of the Interval under mutation.  As presently defined, this Interval library may not support Interval Arithmetic use for this reason.

### Unresolved Static Interval Type Support

In the Boost Interval Container Library mentioned above, two variants of Interval are offered (static compile-time enforced of a single type, or dynamic runtime type supporting mixed Interval types).  The primary downside to static compile-time single types is that some operations are unsupported (e.g. complement) - and many real-world Interval representations require mixed Interval types (e.g. a collection of Intervals representing from -inf to +inf representing the domain of a step function).

### Unresolved Overflow Behavior

No attempt to detect overflow or underflow is applied - can this be done (transforming overflow / underflow to an Unbounded Interval?).

### Unresolved Handling Bounds at Interval Creation

For Intervals with Left and Right Bounds, the user could provide invalid values.  For example, where the Left Bound is greater than the Right Bound, or where the bounds are Equal (in which case this should be a Singleton Interval).  The objective in this library is to catch such errors in a manner that exposes them to the library user, but also to minimize the opportunity for such errors - and to NOT throw exceptions.  Broadly the options encompass:

1. Direct construction of Interval Enum
    * Forces runtime detection of invalid Interval Enum bounds
    * Highly undesirable, as the issue would only be exposed during operations on Intervals
    * How does one prevent this? Simply omit pub on Enum variant definitions?
1. Offer new() constructor
    * Can the target Enum Type be specified to new() - and new returns Interval<T>?
    * If not, we need many new() variants e.g. per Interval Enum Variant?
1. Builder pattern
    * Feels heavyweight for such a simple object, particularly with Variants enumerated

My preference is thew new() option, if it's possible in Rust.  Something like:

```rust
pub fn new(variant: Interval<T>::variant, left_bound: optional<T>, right_bound: optional<T>)
  -> optional<Interval<T>> {
    // Validate bounds are appropriate for variant, else return None
    // Create and return appropriate Interval Variant
}
```

Aside from the sanitization method applied above - there is also the question of detailed behavior when Bounds are invalid or inappropriate for the Interval Variant.  Alternatives considered include:

1. Take in Left(l) and Right(r) Bounds
    * Return error if !(l < r)
    * Else Return Left(l), Right(r)
1. Take in a and b Bounds
    * Return Singleton(a) if a == b
    * Return Left(a), Right(b) if a < b
    * Return Left(b), Right(a) if a > b
1. Take in Left(l) Bound and a Width(w)
    * Return error if w < 0
    * Return Singleton(l) if w == 0
    * Else Return Left(l), Right(l + w)
1. Take in a Bound and Width(w)
    * Return Singleton(a) if w == 0
    * Return Left(a), Right(a+w) if w > 0
    * Return Left(a - w), Right(a) if w < 0
1. Support for an Invalid Interval Enum variant

The question is whether it's better to provide a library which avoids errors so long as a valid interval can be constructed, or whether the interval construction should be providing defense for the programmer when their bounds are not ordered as required.  Reviewing the other Interval libraries in the wild, some provide no validation whatsoever (e.g. [Haskell IntervalMap package](http://hackage.haskell.org/package/IntervalMap), [Boost Interval Container Library - ICL](https://www.boost.org/doc/libs/1_60_0/libs/icl/doc/html/boost_icl/examples/interval.html)).  Others apply the Bound Reversal when mis-ordered (e.g. [Haskell intervals package](http://hackage.haskell.org/package/intervals)). Some Assert if Bound ordering is incorrect (e.g. [Rust Honest Intervals](https://crates.io/crates/honestintervals)).

## Resources

* [Arithmetic operations for floating-point intervals](http://grouper.ieee.org/groups/1788/PositionPapers/ArithOp2.pdf)
* [Interval Arithmetic: from Principles to Implementation](http://fab.cba.mit.edu/classes/S62.12/docs/Hickey_interval.pdf)
* [Union, intersection and complementary of intervals](https://www.sangakoo.com/en/unit/union-intersection-and-complementary-of-intervals)
* [Interval @ Wolfram](http://mathworld.wolfram.com/Interval.html)