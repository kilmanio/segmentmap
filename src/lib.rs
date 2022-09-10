use std::collections::BTreeMap;

use bitmaps::Bitmap;

const SEGMENTSIZE: usize = 64;

pub struct SegmentMap<T: Copy> {
    data: BTreeMap<usize, Segment<T>>,
    first_index: usize,
    last_index: usize,
}

impl<T: Copy> SegmentMap<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            first_index: 0,
            last_index: 0,
        }
    }

    pub fn get(&self, key: usize) -> Option<T> {
        let segment_index = key / SEGMENTSIZE;
        let mantissa = key % SEGMENTSIZE;

        let segment = self.data.get(&segment_index).unwrap();
        segment.get(mantissa)
    }

    pub fn get_mut(&mut self, key: usize) -> Option<T> {
        let segment_index = key / SEGMENTSIZE;
        let mantissa = key % SEGMENTSIZE;

        let segment = self.data.get_mut(&segment_index).unwrap();
        segment.get_mut(mantissa)
    }

    #[must_use]
    pub fn insert(&mut self, item: T) -> usize {
        if self.data.is_empty() {
            self.initialize();
        };
        if self.data.get(&self.last_index).unwrap().is_full() {
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
            }
            //TODO check if this is the 'first' segment and adjust self.first_index accordingly
        }
    }

    pub fn iter() {
        todo!()
    }
}

struct Segment<T: Copy> {
    data: [Option<T>; SEGMENTSIZE],
    bitmap: Bitmap<SEGMENTSIZE>,
    next_index: Option<usize>,
    previous_index: Option<usize>,
}

impl<T: Copy> Segment<T> {
    pub(self) fn new() -> Self {
        Self {
            data: [None; SEGMENTSIZE],
            bitmap: Bitmap::<SEGMENTSIZE>::new(),
            next_index: None,
            previous_index: None,
        }
    }

    fn get_available_index(&self) -> usize {
        match self.bitmap.last_index() {
            Some(index) => index + 1,
            None => 0,
        }
    }
    
    fn get(&self, index: usize) -> Option<T> {
        self.data[index]
    }

    fn get_mut(&mut self, index: usize) -> Option<T> {
        self.data[index]
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
}
