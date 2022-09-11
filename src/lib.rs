#![warn(clippy::pedantic)]
use std::collections::BTreeMap;

use array_macro::array;
use bitmaps::Bitmap;

const SEGMENTSIZE: usize = 32;

#[derive(Clone, Default, Debug)]
pub struct SegmentMap<T> {
    data: BTreeMap<usize, Segment<T>>,
    first_index: usize,
    last_index: usize,
}

impl<T> SegmentMap<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            first_index: 0,
            last_index: 0,
        }
    }

    #[must_use]
    pub fn get(&self, key: usize) -> Option<&T> {
        let segment_index = key / SEGMENTSIZE;
        let mantissa = key % SEGMENTSIZE;

        match self.data.get(&segment_index) {
            Some(segment) => segment.get(mantissa),
            None => None,
        }
    }

    #[must_use]
    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        let segment_index = key / SEGMENTSIZE;
        let mantissa = key % SEGMENTSIZE;

        match self.data.get_mut(&segment_index) {
            Some(segment) => segment.get_mut(mantissa),
            None => None,
        }
    }

    /// # Panics
    ///
    /// if the underlying `BTreeMap` doesn't behave
    #[must_use]
    pub fn insert(&mut self, item: T) -> usize {
        if self.data.is_empty() {
            self.initialize();
        } else if self.data.get(&self.last_index).unwrap().is_full() {
            self.add_new_segment();
        };

        let mantissa = self.data.get_mut(&self.last_index).unwrap().insert(item);
        self.last_index * SEGMENTSIZE + mantissa
    }

    fn add_new_segment(&mut self) {
        let old_last_index = self.last_index;
        self.last_index += 1;
        assert_ne!(old_last_index, self.last_index);
        self.data.insert(self.last_index, Segment::<T>::new());
        self.data.get_mut(&self.last_index).unwrap().previous_index = Some(old_last_index);
        self.data.get_mut(&old_last_index).unwrap().next_index = Some(self.last_index);
    }

    fn initialize(&mut self) {
        self.data.insert(0, Segment::<T>::new());
    }

    /// # Panics
    ///
    /// if the key is not in the map
    //TODO make this error instead
    pub fn remove(&mut self, key: usize) {
        let segment_index = key / SEGMENTSIZE;
        let mantissa = key % SEGMENTSIZE;

        let segment = self.data.get_mut(&segment_index).unwrap();
        segment.remove(mantissa);

        if segment.is_empty() {
            let previous_index = segment.previous_index;
            let next_index = segment.next_index;
            self.data.remove(&segment_index);

            if let Some(index) = next_index {
                self.data.get_mut(&index).unwrap().previous_index = previous_index;
            }

            if let Some(index) = previous_index {
                self.data.get_mut(&index).unwrap().next_index = next_index;
            } else if let Some(index) = next_index {
                self.first_index = index;
            }
        }
    }

    /// Returns an 'Iterator' over all the present items
    ///
    /// # Panics
    ///
    /// If I messed up
    #[must_use]
    pub fn iter(&self) -> SegmentMapIter<T> {
        let first_inner_index = self.data.get(&self.first_index).unwrap().first_index();
        SegmentMapIter {
            segmentmap: self,
            outer_index: Some(self.first_index),
            inner_index: first_inner_index,
        }
    }
}

pub struct SegmentMapIter<'a, T> {
    segmentmap: &'a SegmentMap<T>,
    outer_index: Option<usize>,
    inner_index: Option<usize>,
}

impl<'a, T> Iterator for SegmentMapIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.outer_index?;

        let segment = self.segmentmap.data.get(&self.outer_index?)?;
        let r = segment.get(self.inner_index?);
        self.inner_index = segment.next_index(self.inner_index?);

        if self.inner_index.is_none() {
            self.outer_index = segment.next_index;
            if self.outer_index.is_some() {
                self.inner_index = self.segmentmap.data.get(&self.outer_index?)?.first_index();
            }
        }

        r
    }
}

#[derive(Clone, Copy, Debug, Hash)]
struct Segment<T> {
    data: [Option<T>; SEGMENTSIZE],
    bitmap: Bitmap<SEGMENTSIZE>,
    next_index: Option<usize>,
    previous_index: Option<usize>,
}

impl<T> Segment<T> {
    fn new() -> Self {
        Self {
            data: array!(_ => None; SEGMENTSIZE),
            bitmap: Bitmap::<SEGMENTSIZE>::new(),
            next_index: None,
            previous_index: None,
        }
    }

    fn first_index(&self) -> Option<usize> {
        self.bitmap.first_index()
    }

    fn next_index(&self, index: usize) -> Option<usize> {
        self.bitmap.next_index(index)
    }

    fn get_available_index(&self) -> usize {
        match self.bitmap.last_index() {
            Some(index) => index + 1,
            None => 0,
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data[index].as_ref()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data[index].as_mut()
    }

    fn is_full(&self) -> bool {
        self.bitmap.last_index() == Some(SEGMENTSIZE - 1)
    }

    fn is_empty(&self) -> bool {
        self.bitmap.last_index() == None
    }

    fn insert(&mut self, item: T) -> usize {
        let available_index = self.get_available_index();
        self.bitmap.set(available_index, true);
        self.data[available_index] = Some(item);
        available_index
    }

    fn remove(&mut self, index: usize) {
        self.bitmap.set(index, false);
        self.data[index] = None;
    }
}

#[cfg(test)]
mod tests {
    use crate::{SegmentMap, SEGMENTSIZE};

    #[test]
    fn basic_ops() {
        let mut book = SegmentMap::<bool>::new();
        let index = book.insert(true);
        assert_eq!(index, 0);
        assert!(book.get(index).unwrap());
        book.remove(index);
        assert!(book.get(index).is_none());
    }

    #[test]
    fn mutate() {
        let mut book = SegmentMap::<bool>::new();
        let index = book.insert(true);
        assert_eq!(*book.get(index).unwrap(), true);
        *book.get_mut(index).unwrap() = false;
        assert_eq!(*book.get(index).unwrap(), false);
    }

    #[test]
    fn indices() {
        let inserts = 1000;
        let mut book = SegmentMap::<bool>::new();
        for i in 0..=inserts {
            let index = book.insert(true);
            assert_eq!(index, i);
        }

        let some_index = 512;
        book.remove(some_index);
        assert_eq!(*book.get(0).unwrap(), true);
        assert_eq!(*book.get(some_index - 1).unwrap(), true);
        assert_eq!(*book.get(some_index + 1).unwrap(), true);
        assert_eq!(*book.get(inserts).unwrap(), true);
    }

    #[test]
    fn reuse_inner() {
        let mut book = SegmentMap::<bool>::new();
        let index = book.insert(true);
        assert_eq!(index, 0);
        book.remove(index);
        let index = book.insert(true);
        assert_eq!(index, 0);
    }

    #[test]
    fn reuse_outer() {
        let mut book = SegmentMap::<bool>::new();
        for _ in 0..SEGMENTSIZE {
            let _ = book.insert(true);
        }
        for index in 0..SEGMENTSIZE {
            book.remove(index);
        }
        let index = book.insert(true);
        assert_eq!(index, 0);
    }

    #[test]
    fn iterator() {
        let mut book = SegmentMap::<bool>::new();
        let _ = book.insert(true);
        let _ = book.insert(false);
        let _ = book.insert(true);
        let _ = book.insert(false);
        let mut iter = book.iter();
        assert_eq!(iter.next(), Some(&true));
        assert_eq!(iter.next(), Some(&false));
        assert_eq!(iter.next(), Some(&true));
        assert_eq!(iter.next(), Some(&false));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_gapped() {
        let mut book = SegmentMap::<bool>::new();
        let _ = book.insert(true);
        let a = book.insert(false);
        let _ = book.insert(true);
        let b = book.insert(false);
        book.remove(a);
        book.remove(b);
        let mut iter = book.iter();
        assert_eq!(iter.next(), Some(&true));
        assert_eq!(iter.next(), Some(&true));
        assert_eq!(iter.next(), None);
    }
}
