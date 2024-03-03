use core::{fmt, marker::PhantomData};

use crate::{map, Enum, EnumMap};

/// A set implemented as a [`EnumMap`] where the value is `()`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EnumSet<const LENGTH: usize, E: Enum<LENGTH>>(EnumMap<LENGTH, E, ()>);

impl<const LENGTH: usize, E: Enum<LENGTH>> EnumSet<LENGTH, E> {
    /// Creates an empty `EnumSet`.
    ///
    /// With `debug_assertions` enabled, the constructor verifies the implementation
    /// of the [`Enum`] trait.
    pub fn new() -> Self {
        Self(EnumMap::new())
    }

    /// Clears the set, removing all values.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let mut a = EnumSet::from([Fruit::Orange, Fruit::Banana]);
    ///
    /// assert!(!a.is_empty());
    /// a.clear();
    /// assert!(a.is_empty());
    /// ```

    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns true if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana]);
    ///
    /// assert!(a.contains(Fruit::Orange));
    /// assert!(a.contains(Fruit::Banana));
    /// assert!(!a.contains(Fruit::Grape));
    /// ```
    pub fn contains(&self, value: E) -> bool {
        self.0.contains_key(value)
    }

    /// Visits the values representing the difference, i.e., the values that are in self but not in other.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]);
    /// let b = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Grape]);
    ///
    /// // Can be seen as `a - b`.
    /// for x in a.difference(&b) {
    ///     println!("{x:?}"); // Print Apple
    /// }
    ///
    /// let diff: Vec<Fruit> = a.difference(&b).collect();
    /// assert_eq!(diff, vec![Fruit::Apple]);
    ///
    /// // Note that difference is not symmetric,
    /// // and `b - a` means something else:
    /// let diff: Vec<Fruit> = b.difference(&a).collect();
    /// assert_eq!(diff, vec![Fruit::Grape]);
    /// ```
    pub fn difference<'a>(&'a self, other: &'a EnumSet<LENGTH, E>) -> Difference<'a, LENGTH, E> {
        Difference {
            this: self.0.as_slice(),
            other: other.0.as_slice(),
            index: 0,
            _enum: PhantomData,
        }
    }

    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// If the set did not previously contain this value, true is returned.
    /// If the set already contained this value, false is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumSet;
    ///
    /// let mut set = EnumSet::new();
    ///
    /// assert_eq!(set.insert(Fruit::Orange), true);
    /// assert_eq!(set.insert(Fruit::Orange), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn insert(&mut self, value: E) -> bool {
        self.0.insert(value, ()).is_none()
    }

    /// Visits the values representing the intersection, i.e., the values that are both in self and other.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]);
    /// let b = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Grape]);
    ///
    /// // Print Orange, Banana in order.
    /// for x in a.intersection(&b) {
    ///     println!("{x:?}");
    /// }
    ///
    /// let intersection: Vec<Fruit> = a.intersection(&b).collect();
    /// assert_eq!(intersection, vec![Fruit::Orange, Fruit::Banana]);
    /// ```
    pub fn intersection<'a>(
        &'a self,
        other: &'a EnumSet<LENGTH, E>,
    ) -> Intersection<'a, LENGTH, E> {
        Intersection {
            this: self.0.as_slice(),
            other: other.0.as_slice(),
            index: 0,
            _enum: PhantomData,
        }
    }

    /// Returns true if self has no elements in common with other.
    /// This is equivalent to checking for an empty intersection.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana]);
    /// let mut b = EnumSet::new();
    ///
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(Fruit::Grape);
    /// assert_eq!(a.is_disjoint(&b), true);
    /// b.insert(Fruit::Orange);
    /// assert_eq!(a.is_disjoint(&b), false);
    /// ```
    pub fn is_disjoint(&self, other: &EnumSet<LENGTH, E>) -> bool {
        self.intersection(other).next().is_none()
    }

    /// Returns true if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumSet;
    ///
    /// let mut set = EnumSet::new();
    /// assert!(set.is_empty());
    /// set.insert(Fruit::Orange);
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if the set is a subset of another, i.e.,
    /// other contains at least all the values in self.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let sup = EnumSet::from([Fruit::Orange, Fruit::Banana]);
    /// let mut set = EnumSet::new();
    ///
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(Fruit::Orange);
    /// assert_eq!(set.is_subset(&sup), true);
    /// set.insert(Fruit::Grape);
    /// assert_eq!(set.is_subset(&sup), false);
    /// ```
    pub fn is_subset(&self, other: &EnumSet<LENGTH, E>) -> bool {
        self.difference(other).next().is_none()
    }

    /// Returns true if the set is a superset of another, i.e.,
    /// self contains at least all the values in other.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let sub = EnumSet::from([Fruit::Orange, Fruit::Banana]);
    /// let mut set = EnumSet::new();
    ///
    /// assert_eq!(set.is_superset(&sub), false);
    /// set.insert(Fruit::Orange);
    /// assert_eq!(set.is_superset(&sub), false);
    /// set.insert(Fruit::Banana);
    /// assert_eq!(set.is_superset(&sub), true);
    /// set.insert(Fruit::Grape);
    /// assert_eq!(set.is_superset(&sub), true);
    /// ```
    pub fn is_superset(&self, other: &EnumSet<LENGTH, E>) -> bool {
        other.difference(self).next().is_none()
    }

    /// An iterator visiting all elements in order. The iterator element type is `E`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug)] enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumSet;
    ///
    /// let mut set = EnumSet::from([
    ///     Fruit::Orange,
    ///     Fruit::Grape,
    /// ]);
    ///
    /// for value in set.iter() {
    ///     println!("{value:?}");
    /// }
    /// # let mut iter = set.iter();
    /// # assert!(matches!(iter.next(), Some(Fruit::Orange)));
    /// # assert!(matches!(iter.next(), Some(Fruit::Grape)));
    /// # assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> Iter<'_, LENGTH, E> {
        Iter {
            inner: self.0.keys(),
        }
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumSet;
    ///
    /// let mut set = EnumSet::new();
    /// assert_eq!(set.len(), 0);
    /// set.insert(Fruit::Grape);
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Removes a value from the set. Returns whether the value was present in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumSet;
    ///
    /// let mut set = EnumSet::new();
    ///
    /// set.insert(Fruit::Orange);
    /// assert_eq!(set.remove(Fruit::Orange), true);
    /// assert_eq!(set.remove(Fruit::Orange), false);
    /// assert!(set.is_empty());
    /// ```
    pub fn remove(&mut self, value: E) -> bool {
        self.0.remove(value).is_some()
    }

    /// Visits the values representing the union, i.e.,
    /// all the values in self or other, without duplicates.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]);
    /// let b = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Grape]);
    ///
    /// // Print Orange, Banana, Grape, Apple in order.
    /// for x in a.union(&b) {
    ///     println!("{x:?}");
    /// }
    ///
    /// let union: Vec<Fruit> = a.union(&b).collect();
    /// assert_eq!(union, vec![Fruit::Orange, Fruit::Banana, Fruit::Grape, Fruit::Apple]);
    /// ```
    pub fn union<'a>(&'a self, other: &'a EnumSet<LENGTH, E>) -> Union<'a, LENGTH, E> {
        Union {
            this: self.0.as_slice(),
            other: other.0.as_slice(),
            index: 0,
            _enum: PhantomData,
        }
    }

    /// Visits the values representing the symmetric difference, i.e.,
    /// the values that are in self or in other but not in both.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]);
    /// let b = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Grape]);
    ///
    /// // Print Grape, Apple in order.
    /// for x in a.symmetric_difference(&b) {
    ///     println!("{x:?}");
    /// }
    ///
    /// let diff1: Vec<Fruit> = a.symmetric_difference(&b).collect();
    /// let diff2: Vec<Fruit> = b.symmetric_difference(&a).collect();
    /// assert_eq!(diff1, diff2);
    /// assert_eq!(diff1, vec![Fruit::Grape, Fruit::Apple]);
    /// ```
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a EnumSet<LENGTH, E>,
    ) -> SymmetricDifference<'a, LENGTH, E> {
        SymmetricDifference {
            this: self.0.as_slice(),
            other: other.0.as_slice(),
            index: 0,
            _enum: PhantomData,
        }
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> Default for EnumSet<LENGTH, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> fmt::Debug for EnumSet<LENGTH, E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>, const N: usize> From<[E; N]> for EnumSet<LENGTH, E> {
    fn from(value: [E; N]) -> Self {
        Self::from_iter(value)
    }
}

/// Converts an `EnumMap` into an `EnumSet` containing all of the map's keys.
///
/// # Examples
///
/// ```
/// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, } }
/// use enumap::{EnumMap, EnumSet};
///
/// let map = EnumMap::from([(Fruit::Banana, 100), (Fruit::Grape, 200)]);
/// let set = EnumSet::from(map);
///
/// assert!(set.contains(Fruit::Banana));
/// assert!(set.contains(Fruit::Grape));
/// assert!(!set.contains(Fruit::Orange));
/// ```
impl<const LENGTH: usize, E: Enum<LENGTH>, V> From<EnumMap<LENGTH, E, V>> for EnumSet<LENGTH, E> {
    fn from(value: EnumMap<LENGTH, E, V>) -> Self {
        let data: [_; LENGTH] = value.into();
        Self(data.map(|v| v.map(|_| ())).into())
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> FromIterator<E> for EnumSet<LENGTH, E> {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>> FromIterator<&'a E> for EnumSet<LENGTH, E> {
    fn from_iter<T: IntoIterator<Item = &'a E>>(iter: T) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> Extend<E> for EnumSet<LENGTH, E> {
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        for value in iter {
            self.insert(value);
        }
    }
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>> Extend<&'a E> for EnumSet<LENGTH, E> {
    fn extend<T: IntoIterator<Item = &'a E>>(&mut self, iter: T) {
        for value in iter {
            self.insert(*value);
        }
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> IntoIterator for EnumSet<LENGTH, E> {
    type Item = E;
    type IntoIter = IntoIter<LENGTH, E>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.0.into_iter(),
        }
    }
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>> IntoIterator for &'a EnumSet<LENGTH, E> {
    type Item = E;
    type IntoIter = Iter<'a, LENGTH, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> core::ops::BitAnd<&EnumSet<LENGTH, E>>
    for &EnumSet<LENGTH, E>
{
    type Output = EnumSet<LENGTH, E>;

    /// Returns the intersection of `self` and `rhs` as a new `EnumSet<LENGTH, E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]);
    /// let b = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Grape]);
    ///
    /// let set = &a & &b;
    /// assert_eq!(set, EnumSet::from([Fruit::Orange, Fruit::Banana]));
    /// ```
    fn bitand(self, rhs: &EnumSet<LENGTH, E>) -> Self::Output {
        self.intersection(rhs).collect()
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> core::ops::BitAnd<E> for EnumSet<LENGTH, E> {
    type Output = EnumSet<LENGTH, E>;

    /// Returns the intersection of `self` and `rhs` as a new `EnumSet<LENGTH, E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let set = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]);
    ///
    /// let set = set & Fruit::Apple;
    /// assert_eq!(set, EnumSet::from([Fruit::Apple]));
    ///
    /// let set = set & Fruit::Grape;
    /// assert_eq!(set, EnumSet::new());
    /// ```
    fn bitand(mut self, rhs: E) -> Self::Output {
        if self.contains(rhs) {
            self.clear();
            self.insert(rhs);
        } else {
            self.clear();
        }
        self
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> core::ops::BitOr<&EnumSet<LENGTH, E>>
    for &EnumSet<LENGTH, E>
{
    type Output = EnumSet<LENGTH, E>;

    /// Returns the union of `self` and `rhs` as a new `EnumSet<LENGTH, E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Apple]);
    /// let b = EnumSet::from([Fruit::Orange, Fruit::Grape]);
    ///
    /// let set = &a | &b;
    /// assert_eq!(set, EnumSet::from([Fruit::Orange, Fruit::Grape, Fruit::Apple]));
    /// ```
    fn bitor(self, rhs: &EnumSet<LENGTH, E>) -> Self::Output {
        self.union(rhs).collect()
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> core::ops::BitOr<E> for EnumSet<LENGTH, E> {
    type Output = EnumSet<LENGTH, E>;

    /// Returns the union of `self` and `rhs` as a new `EnumSet<LENGTH, E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let set = EnumSet::from([Fruit::Orange, Fruit::Apple]);
    ///
    /// let set = set | Fruit::Banana | Fruit::Orange;
    /// assert_eq!(set, EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]));
    /// ```
    fn bitor(mut self, rhs: E) -> Self::Output {
        self.insert(rhs);
        self
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> core::ops::BitXor<&EnumSet<LENGTH, E>>
    for &EnumSet<LENGTH, E>
{
    type Output = EnumSet<LENGTH, E>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `EnumSet<LENGTH, E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]);
    /// let b = EnumSet::from([Fruit::Orange, Fruit::Grape]);
    ///
    /// let set = &a ^ &b;
    /// assert_eq!(set, EnumSet::from([Fruit::Banana, Fruit::Grape, Fruit::Apple]));
    /// ```
    fn bitxor(self, rhs: &EnumSet<LENGTH, E>) -> Self::Output {
        self.symmetric_difference(rhs).collect()
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> core::ops::BitXor<E> for EnumSet<LENGTH, E> {
    type Output = EnumSet<LENGTH, E>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `EnumSet<LENGTH, E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let set = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Grape]);
    ///
    /// let set = set ^ Fruit::Banana;
    /// assert_eq!(set, EnumSet::from([Fruit::Orange, Fruit::Grape]));
    ///
    /// let set = set ^ Fruit::Banana;
    /// assert_eq!(set, EnumSet::from([Fruit::Banana]));
    /// ```
    fn bitxor(mut self, rhs: E) -> Self::Output {
        if !self.remove(rhs) {
            EnumSet::from([rhs])
        } else {
            self
        }
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>> core::ops::Sub<&EnumSet<LENGTH, E>>
    for &EnumSet<LENGTH, E>
{
    type Output = EnumSet<LENGTH, E>;

    /// Returns the difference of `self` and `rhs` as a new `EnumSet<LENGTH, E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug, PartialEq)] enum Fruit { Orange, Banana, Grape, Apple } }
    /// use enumap::{Enum, EnumSet};
    ///
    /// let a = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Apple]);
    /// let b = EnumSet::from([Fruit::Orange, Fruit::Banana, Fruit::Grape]);
    ///
    /// let set = &a - &b;
    /// assert_eq!(set, EnumSet::from([Fruit::Apple]));
    /// ```
    fn sub(self, rhs: &EnumSet<LENGTH, E>) -> Self::Output {
        self.difference(rhs).collect()
    }
}

/// Iterator returned from [`EnumSet::iter`].
pub struct Iter<'a, const LENGTH: usize, E: Enum<LENGTH>> {
    inner: map::Keys<'a, LENGTH, E, ()>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>> Iterator for Iter<'a, LENGTH, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Iterator returned from [`EnumSet::into_iter`].
pub struct IntoIter<const LENGTH: usize, E: Enum<LENGTH>> {
    inner: map::IntoIter<LENGTH, E, ()>,
}

impl<const LENGTH: usize, E: Enum<LENGTH>> Iterator for IntoIter<LENGTH, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(v, _)| v)
    }
}

/// Iterator returned from [`EnumSet::difference`].
pub struct Difference<'a, const LENGTH: usize, E: Enum<LENGTH>> {
    this: &'a [Option<()>; LENGTH],
    other: &'a [Option<()>; LENGTH],
    index: usize,
    _enum: PhantomData<E>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>> Iterator for Difference<'a, LENGTH, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < LENGTH {
            let index = self.index;
            self.index += 1;

            if self.this[index].is_some() && self.other[index].is_none() {
                return E::from_index(index);
            }
        }

        None
    }
}

/// Iterator returned from [`EnumSet::intersection`].
pub struct Intersection<'a, const LENGTH: usize, E: Enum<LENGTH>> {
    this: &'a [Option<()>; LENGTH],
    other: &'a [Option<()>; LENGTH],
    index: usize,
    _enum: PhantomData<E>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>> Iterator for Intersection<'a, LENGTH, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < LENGTH {
            let index = self.index;
            self.index += 1;

            if self.this[index].is_some() && self.other[index].is_some() {
                return E::from_index(index);
            }
        }

        None
    }
}

/// Iterator returned from [`EnumSet::union`].
pub struct Union<'a, const LENGTH: usize, E: Enum<LENGTH>> {
    this: &'a [Option<()>; LENGTH],
    other: &'a [Option<()>; LENGTH],
    index: usize,
    _enum: PhantomData<E>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>> Iterator for Union<'a, LENGTH, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < LENGTH {
            let index = self.index;
            self.index += 1;

            if self.this[index].is_some() || self.other[index].is_some() {
                return E::from_index(index);
            }
        }

        None
    }
}

/// Iterator returned from [`EnumSet::symmetric_difference`].
pub struct SymmetricDifference<'a, const LENGTH: usize, E: Enum<LENGTH>> {
    this: &'a [Option<()>; LENGTH],
    other: &'a [Option<()>; LENGTH],
    index: usize,
    _enum: PhantomData<E>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>> Iterator for SymmetricDifference<'a, LENGTH, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < LENGTH {
            let index = self.index;
            self.index += 1;

            if self.this[index].is_some() ^ self.other[index].is_some() {
                return E::from_index(index);
            }
        }

        None
    }
}
