use crate::{IntMap, IntoIter};
use std::{fmt::Debug, iter::FromIterator};

#[derive(Clone)]
#[cfg_attr(feature = "with_serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntSet {
    int_map: IntMap<()>,
}

impl IntSet {
    #[inline]
    pub fn new() -> Self {
        IntSet {
            int_map: IntMap::new(),
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        IntSet {
            int_map: IntMap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn insert(&mut self, val: u32) -> bool {
        self.int_map.insert_checked(val, ())
    }

    #[inline]
    pub fn remove(&mut self, val: u32) -> bool {
        self.int_map.remove(val).is_some()
    }

    #[inline]
    pub fn contains(&self, val: u32) -> bool {
        self.int_map.contains_key(val)
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.int_map.reserve(additional)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.int_map.clear()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.int_map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.int_map.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = u32> + '_ {
        self.int_map.iter().map(|i| *i.0)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.int_map.capacity()
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(u32) -> bool,
    {
        self.int_map.retain(|k, _| f(k))
    }
}

impl FromIterator<u32> for IntSet {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut new = IntSet::new();
        for i in iter {
            new.insert(i);
        }
        new
    }
}

impl Extend<u32> for IntSet {
    #[inline]
    fn extend<T: IntoIterator<Item = u32>>(&mut self, iter: T) {
        for i in iter {
            self.insert(i);
        }
    }
}

impl IntoIterator for IntSet {
    type Item = u32;

    type IntoIter = IntoIteratorIS;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIteratorIS {
            set: self.int_map.into_iter(),
        }
    }
}

/// IntoIter for IntSet
pub struct IntoIteratorIS {
    set: IntoIter<u32, ()>,
}

impl Iterator for IntoIteratorIS {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.set.next().map(|i| i.0)
    }
}

impl PartialEq for IntSet {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().all(|j| other.contains(j))
    }
}

impl Eq for IntSet {}

impl Debug for IntSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Default for IntSet {
    #[inline]
    fn default() -> Self {
        IntSet::new()
    }
}
