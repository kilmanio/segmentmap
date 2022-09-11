use segmentmap::SegmentMap;
use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

fn main() {
    let iterations = 0..1_000_000;
    let mut segmentmap = SegmentMap::<bool>::new();
    let mut hashmap = HashMap::new();
    let mut btreemap = BTreeMap::new();
    let mut vec = Vec::new();

    {
        println!("SegmentMap");
        let now = Instant::now();
        for _ in iterations.clone() {
            segmentmap.insert(true);
        }
        let elapsed = now.elapsed();
        println!(" - Adding 1M booleans: {:.2?}", elapsed);

        let now = Instant::now();
        let mut iter = segmentmap.iter();
        while let Some(_) = iter.next() {}
        let elapsed = now.elapsed();
        println!(" - Iterating over 1M booleans: {:.2?}", elapsed);

        let now = Instant::now();
        let mut iter = segmentmap.iter_with_index();
        while let Some(_) = iter.next() {}
        let elapsed = now.elapsed();
        println!(" - Iterating over 1M booleans, with index: {:.2?}", elapsed);

        let now = Instant::now();
        for i in iterations.clone() {
            segmentmap.remove(i);
        }
        let elapsed = now.elapsed();
        println!(" - Sequentially Deleting 1M booleans: {:.2?}", elapsed);
    }

    {
        println!("HashMap");
        let now = Instant::now();
        for i in iterations.clone() {
            hashmap.insert(i, true);
        }
        let elapsed = now.elapsed();
        println!(" - Adding 1M booleans: {:.2?}", elapsed);

        let now = Instant::now();
        let mut iter = hashmap.iter();
        while let Some(_) = iter.next() {}
        let elapsed = now.elapsed();
        println!(" - Iterating over 1M booleans: {:.2?}", elapsed);

        let now = Instant::now();
        for i in iterations.clone() {
            hashmap.remove(&i);
        }
        let elapsed = now.elapsed();
        println!(" - Sequentially Deleting 1M booleans: {:.2?}", elapsed);
    }

    {
        println!("BTreeMap");
        let now = Instant::now();
        for i in iterations.clone() {
            btreemap.insert(i, true);
        }
        let elapsed = now.elapsed();
        println!(" - Adding 1M booleans: {:.2?}", elapsed);

        let now = Instant::now();
        let mut iter = btreemap.iter();
        while let Some(_) = iter.next() {}
        let elapsed = now.elapsed();
        println!(" - Iterating over 1M booleans: {:.2?}", elapsed);

        let now = Instant::now();
        for i in iterations.clone() {
            btreemap.remove(&i);
        }
        let elapsed = now.elapsed();
        println!(" - Sequentially Deleting 1M booleans: {:.2?}", elapsed);
    }

    {
        println!("Vec");
        let now = Instant::now();
        for i in iterations.clone() {
            vec.insert(i, true);
        }
        let elapsed = now.elapsed();
        println!(" - Adding 1M booleans: {:.2?}", elapsed);

        let now = Instant::now();
        let mut iter = vec.iter();
        while let Some(_) = iter.next() {}
        let elapsed = now.elapsed();
        println!(" - Iterating over 1M booleans: {:.2?}", elapsed);

        let now = Instant::now();
        for _ in iterations.clone() {
            vec.remove(0);
        }
        let elapsed = now.elapsed();
        println!(" - Sequentially Deleting 1M booleans: {:.2?}", elapsed);
    }
}
