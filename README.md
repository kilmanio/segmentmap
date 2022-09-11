# Segmentmap

A generic collection that preserves the order of items at all cost.
Removing an item never restructures, keys/indices are unique per item.
Internally uses a HashMap of arrays containing the items.
Also tested with an internal BTreeMap, but results were worse.
Iterator is stable, using a linked-list-like structure.

Using an array size >128 makes the segmentmap significantly slower when inserting (>10%).
Similar when you use a small array size (like 16).
The smaller the array, the more similar it is to an [indexmap](https://lib.rs/crates/indexmap),
The larger the array, the more similar it is to an array (duh).
(but in both cases it will still preserve order when removing, without restructuring).

More of a PoC than something you should start using.

# Benchmarking
These are the results of some very basic benchmarking I did (cargo run) against common collections.
Listed are the durations of doing 1 million operations sequentially, so to get the average just divide by 1M.

| type | adding 1M | iterating over 1M | deleting 1M sequentially |
|------|-----------|-------------------|--------------------------|
| SegmentMap | 654.97ms | 305.44ms | 312.97ms |
| HashMap | 983.93ms | 37.15ms | 579.11ms |
| BTreeMap | 2.48s | 97.58ms | 1.07s |
| Vec | 36.67ms | 17.45ms | 10.25s |

