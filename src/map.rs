use core::marker::PhantomData;

use crate::Enum;

/// An enum map backed by an array.
///
/// The map is backed by `[Option<V>; E::LENGTH]`, which means it does not allocate,
/// but depending on the length of the enum and the size of the `V` it can require a significant
/// amount of space. In some cases it may be beneficial to box the enum map.
///
/// To reduce the amount of space required, consider using values with a niche, like `NonZeroUsize`.
///
/// An incorrectly implemented [`Enum`] trait will not cause undefined behaviour but
/// may introduce random panics and incorrect results. Consider using the [`enumap`](crate::enumap)
/// macro to implement [`Enum`] correctly.
#[derive(Clone, Copy)]
pub struct EnumMap<const LENGTH: usize, E: Enum<LENGTH>, V> {
    data: [Option<V>; LENGTH],
    _enum: PhantomData<E>,
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> EnumMap<LENGTH, E, V> {
    /// Creates an empty `EnumMap`.
    ///
    /// With `debug_assertions` enabled, the constructor verifies the implementation
    /// of the [`Enum`] trait.
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        assert_enum_impl::<LENGTH, E>();

        Self {
            data: [(); LENGTH].map(|_| None),
            _enum: PhantomData,
        }
    }

    /// Clears the map, removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::new();
    ///
    /// map.insert(Fruit::Orange, 3);
    /// assert!(map.contains_key(Fruit::Orange));
    ///
    /// map.clear();
    /// assert!(!map.contains_key(Fruit::Orange));
    pub fn clear(&mut self) {
        self.data = [(); LENGTH].map(|_| None);
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::new();
    /// map.insert(Fruit::Orange, 3);
    ///
    /// assert!(map.contains_key(Fruit::Orange));
    /// assert!(!map.contains_key(Fruit::Banana));
    /// ```
    pub fn contains_key(&self, key: E) -> bool {
        self.get(key).is_some()
    }

    /// Returns a reference to the value for the corresponding key.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::new();
    /// map.insert(Fruit::Orange, 3);
    ///
    /// assert_eq!(map.get(Fruit::Orange), Some(&3));
    /// assert_eq!(map.get(Fruit::Banana), None);
    /// ```
    pub fn get(&self, key: E) -> Option<&V> {
        self.data[E::to_index(key)].as_ref()
    }

    /// Returns a mutable reference to the value for the corresponding key.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::new();
    /// map.insert(Fruit::Orange, 3);
    ///
    /// if let Some(value) = map.get_mut(Fruit::Orange) {
    ///     *value += 2;
    /// }
    /// assert_eq!(map[Fruit::Orange], 5);
    /// ```
    pub fn get_mut(&mut self, key: E) -> Option<&mut V> {
        self.data[E::to_index(key)].as_mut()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map already had a value present for the key,
    /// the old value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::new();
    /// assert_eq!(map.insert(Fruit::Orange, 3), None);
    /// assert_eq!(map.insert(Fruit::Orange, 5), Some(3));
    /// ```
    pub fn insert(&mut self, key: E, value: V) -> Option<V> {
        core::mem::replace(&mut self.data[E::to_index(key)], Some(value))
    }

    /// Creates a consuming iterator visiting all the values in order.
    /// The map cannot be used after calling this. The iterator element type is V.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::from([
    ///     (Fruit::Grape, 3),
    ///     (Fruit::Banana, 2),
    ///     (Fruit::Orange, 1),
    /// ]);
    ///
    /// let vec: Vec<i32> = map.into_values().collect();
    /// assert_eq!(vec, vec![1, 2, 3]);
    /// ```
    pub fn into_values(self) -> IntoValues<LENGTH, E, V> {
        IntoValues {
            inner: self.data.into_iter().filter(Option::is_some),
            _enum: PhantomData,
        }
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::new();
    /// assert!(map.is_empty());
    /// map.insert(Fruit::Orange, 3);
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(Option::is_none)
    }

    /// An iterator visiting all key-value pairs in order, with references to the values.
    /// The iterator element type is (E, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug)] enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::from([
    ///     (Fruit::Orange, 1),
    ///     (Fruit::Banana, 2),
    ///     (Fruit::Grape, 3),
    /// ]);
    ///
    /// for (key, value) in map.iter() {
    ///     println!("key: {key:?} value: {value}");
    /// }
    /// # for (i, (k, value)) in map.iter().enumerate() {
    /// #     assert_eq!(*value, i + 1);
    /// #     assert_eq!(*value, map[k]);
    /// # }
    /// ```
    pub fn iter(&self) -> Iter<'_, LENGTH, E, V> {
        Iter {
            map: self,
            index: 0,
        }
    }

    /// An iterator visiting all key-value pairs in order, with mutable references to the values.
    /// The iterator element type is (E, &'a mut V).
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::from([
    ///     (Fruit::Orange, 1),
    ///     (Fruit::Banana, 2),
    ///     (Fruit::Grape, 3),
    /// ]);
    ///
    /// for (_, value) in map.iter_mut() {
    ///     *value *= 2;
    /// }
    ///
    /// assert_eq!(map[Fruit::Orange], 2);
    /// assert_eq!(map[Fruit::Banana], 4);
    /// assert_eq!(map[Fruit::Grape], 6);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, LENGTH, E, V> {
        IterMut {
            data: Some(&mut self.data),
            index: 0,
            _enum: PhantomData,
        }
    }

    /// An iterator visiting all keys in order. The iterator element type is `E`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug)] enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::from([
    ///     (Fruit::Orange, 1),
    ///     (Fruit::Grape, 2),
    /// ]);
    ///
    /// for key in map.keys() {
    ///     println!("{key:?}");
    /// }
    /// # let mut iter = map.keys();
    /// # assert!(matches!(iter.next(), Some(Fruit::Orange)));
    /// # assert!(matches!(iter.next(), Some(Fruit::Grape)));
    /// # assert!(iter.next().is_none());
    /// ```
    pub fn keys(&self) -> Keys<'_, LENGTH, E, V> {
        Keys { inner: self.iter() }
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert(Fruit::Orange, "a");
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.data.iter().filter(|v| v.is_some()).count()
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::new();
    /// map.insert(Fruit::Orange, "a");
    /// assert_eq!(map.remove(Fruit::Orange), Some("a"));
    /// ```
    pub fn remove(&mut self, key: E) -> Option<V> {
        core::mem::take(&mut self.data[E::to_index(key)])
    }

    /// An iterator visiting all values in order. The iterator element type is `&'a V`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug)] enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::from([
    ///     (Fruit::Orange, 1),
    ///     (Fruit::Grape, 2),
    /// ]);
    ///
    /// for value in map.values() {
    ///     println!("{value:?}");
    /// }
    /// # let mut iter = map.values();
    /// # assert!(matches!(iter.next(), Some(1)));
    /// # assert!(matches!(iter.next(), Some(2)));
    /// # assert!(iter.next().is_none());
    /// ```
    pub fn values(&self) -> Values<'_, LENGTH, E, V> {
        Values { inner: self.iter() }
    }

    /// An iterator visiting all values mutably in order. The iterator element type is `&'a mut V`.
    ///
    /// # Examples
    ///
    /// ```
    /// # enumap::enumap! { #[derive(Debug)] enum Fruit { Orange, Banana, Grape, } }
    /// use enumap::EnumMap;
    ///
    /// let mut map = EnumMap::from([
    ///     (Fruit::Orange, 1),
    ///     (Fruit::Grape, 2),
    /// ]);
    ///
    /// for value in map.values_mut() {
    ///     *value += 10;
    /// }
    ///
    /// assert_eq!(map[Fruit::Orange], 11);
    /// assert_eq!(map[Fruit::Grape], 12);
    /// # let mut iter = map.values_mut();
    /// # assert!(matches!(iter.next(), Some(11)));
    /// # assert!(matches!(iter.next(), Some(12)));
    /// # assert!(iter.next().is_none());
    /// ```
    pub fn values_mut(&mut self) -> ValuesMut<'_, LENGTH, E, V> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> Default for EnumMap<LENGTH, E, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V, const N: usize> From<[(E, V); N]>
    for EnumMap<LENGTH, E, V>
{
    fn from(value: [(E, V); N]) -> Self {
        Self::from_iter(value)
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> FromIterator<(E, V)> for EnumMap<LENGTH, E, V> {
    fn from_iter<T: IntoIterator<Item = (E, V)>>(iter: T) -> Self {
        let mut map = Self::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> core::ops::Index<E> for EnumMap<LENGTH, E, V> {
    type Output = V;

    fn index(&self, index: E) -> &Self::Output {
        self.get(index).expect("no entry found for key")
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> IntoIterator for EnumMap<LENGTH, E, V> {
    type Item = (E, V);
    type IntoIter = IntoIter<LENGTH, E, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::<LENGTH, E, V>::new(self)
    }
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>, V> IntoIterator for &'a EnumMap<LENGTH, E, V> {
    type Item = (E, &'a V);
    type IntoIter = Iter<'a, LENGTH, E, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, const LENGTH: usize, E: Enum<LENGTH>, V> {
    index: usize,
    map: &'a EnumMap<LENGTH, E, V>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>, V> Iterator for Iter<'a, LENGTH, E, V> {
    type Item = (E, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.map.data.len() {
            let index = self.index;
            self.index += 1;

            if let Some(value) = &self.map.data[index] {
                return Some((E::from_index(index)?, value));
            }
        }

        None
    }
}

pub struct Keys<'a, const LENGTH: usize, E: Enum<LENGTH>, V> {
    inner: Iter<'a, LENGTH, E, V>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>, V> Iterator for Keys<'a, LENGTH, E, V> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }
}

pub struct Values<'a, const LENGTH: usize, E: Enum<LENGTH>, V> {
    inner: Iter<'a, LENGTH, E, V>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>, V> Iterator for Values<'a, LENGTH, E, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }
}

pub struct ValuesMut<'a, const LENGTH: usize, E: Enum<LENGTH>, V> {
    inner: IterMut<'a, LENGTH, E, V>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>, V> Iterator for ValuesMut<'a, LENGTH, E, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }
}

#[allow(clippy::type_complexity)]
pub struct IntoValues<const LENGTH: usize, E: Enum<LENGTH>, V> {
    inner: core::iter::Filter<core::array::IntoIter<Option<V>, LENGTH>, fn(&Option<V>) -> bool>,
    _enum: PhantomData<E>,
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> Iterator for IntoValues<LENGTH, E, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().flatten()
    }
}

pub struct IterMut<'a, const LENGTH: usize, E: Enum<LENGTH>, V> {
    data: Option<&'a mut [Option<V>]>,
    index: usize,
    _enum: PhantomData<E>,
}

impl<'a, const LENGTH: usize, E: Enum<LENGTH>, V> Iterator for IterMut<'a, LENGTH, E, V> {
    type Item = (E, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.data.take().and_then(|mut data| {
            let mut index = self.index;
            let (element, tail) = loop {
                let (head, tail) = data.split_first_mut()?;
                self.index += 1;
                if let Some(head) = head {
                    break (head, tail);
                }
                index += 1;
                data = tail;
            };
            self.data = Some(tail);
            Some((E::from_index(index)?, element))
        })
    }
}

pub struct IntoIter<const LENGTH: usize, E: Enum<LENGTH>, V> {
    index: usize,
    map: EnumMap<LENGTH, E, V>,
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> IntoIter<LENGTH, E, V> {
    fn new(map: EnumMap<LENGTH, E, V>) -> Self {
        Self { index: 0, map }
    }
}

impl<const LENGTH: usize, E: Enum<LENGTH>, V> Iterator for IntoIter<LENGTH, E, V> {
    type Item = (E, V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.map.data.len() {
            let index = self.index;
            self.index += 1;

            let value = core::mem::take(&mut self.map.data[index]);
            if let Some(value) = value {
                return Some((E::from_index(index)?, value));
            }
        }

        None
    }
}

#[cfg(debug_assertions)]
fn assert_enum_impl<const LENGTH: usize, E>()
where
    E: Enum<LENGTH>,
{
    let ty = core::any::type_name::<E>();
    for i in 0..LENGTH + 1 {
        let Some(v) = E::from_index(i) else {
            assert_eq!(
                i, LENGTH,
                "No variant constructed from index {i} for enum {ty} with LENGTH {LENGTH}",
            );
            return;
        };

        assert_eq!(
            i, E::to_index(v),
            "`to_index` returned different index for variant constructed at index {i} with `from_index`",
        );
    }

    panic!("Enum {ty} yielded more variants from `from_index` than LENGTH ({LENGTH})");
}