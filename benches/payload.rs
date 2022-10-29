use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use segmentmap::SegmentMap;

///This is supposed to represent a payload of some size, that might be more realistic than a bool
#[derive(Clone, Copy)]
struct Payload {
    _data: (u128, u128, u128, u128, u128),
}

impl Payload {
    fn blank() -> Self {
        Self {
            _data: (0, 0, 0, 0, 0),
        }
    }

    fn new(i: u128) -> Self {
        Self {
            _data: (i, i, i, i, i),
        }
    }
}

fn setup() -> SegmentMap<Payload> {
    let mut map = SegmentMap::<Payload>::new();
    for i in 0..1_000 {
        let _ = map.insert(Payload::new(i));
    }
    map
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let map = setup();
    let index = 1 * segmentmap::SEGMENTSIZE + 5;
    c.bench_function("Remove", |b| {
        b.iter_batched_ref(
            || map.clone(),
            |cloned_map| cloned_map.remove(black_box(index)),
            BatchSize::SmallInput,
        )
    });
    let payload = Payload::blank();
    c.bench_function("Add", |b| {
        b.iter_batched_ref(
            || map.clone(),
            |cloned_map| cloned_map.insert(black_box(payload)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
