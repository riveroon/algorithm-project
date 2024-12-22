pub use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
pub use fastrand::Rng;

pub struct Settings {
    pub count: usize,
    pub range: u32
}

pub const SMALL_COUNT: Settings = Settings {
    count: 400, 
    range: 600
};

pub const LARGE_COUNT: Settings = Settings {
    count: 4_000_000,
    range: 6_000_000
};

#[derive(PartialEq, Eq, Hash)]
pub struct SmallKey(u32);

impl SmallKey {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn rand(rng: &mut Rng, range: u32) -> Self {
        Self::new(rng.u32(..range))
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct LargeKey {
    id: u32,
    data: [u8; 256]
}

impl LargeKey {
    pub fn new(id: u32) -> Self {
        let data = [0; 256];
        Self { id, data }
    }

    pub fn rand(rng: &mut Rng, range: u32) -> Self {
        let mut this = Self::new(rng.u32(..range));
        rng.fill(&mut this.data);

        this
    }
}

#[macro_export]
macro_rules! setup {
    ($map:ty, $rng:ident, $iter:ident, $key:ty) => {
        {
            const SETTINGS: $crate::Settings = $crate::$iter;
            let mut map = <$map>::with_capacity(SETTINGS.count);
            for i in 0..SETTINGS.count {
                map.insert(<$key>::rand(&mut $rng, SETTINGS.range), i);
            }
            map
        }
    };
}

#[macro_export]
macro_rules! insert_one {
    ($label:literal, $group:ident, $map:ty, $rng:ident, $iter:ident, $key:ty) => {
        $group.bench_with_input($label, &$rng, |b, rng| {
            const SETTINGS: $crate::Settings = $crate::$iter;
            let mut rng = rng.clone();
            b.iter(|| {
                let mut map = <$map>::with_capacity(SETTINGS.count);
                for i in 0..SETTINGS.count {
                    map.insert(black_box(<$key>::rand(&mut rng, SETTINGS.range)), i);
                }
            })
        });
    };
}

#[macro_export]
macro_rules! insertion {
    ($group:ident, $map:ty, $rng:ident) => {
        $crate::insert_one!("insert_small_few", $group, $map, $rng, SMALL_COUNT, SmallKey);
        $crate::insert_one!("insert_small_many", $group, $map, $rng, LARGE_COUNT, SmallKey);
        $crate::insert_one!("insert_large_few", $group, $map, $rng, SMALL_COUNT, LargeKey);
        $crate::insert_one!("insert_large_many", $group, $map, $rng, LARGE_COUNT, LargeKey);
    };
}

#[macro_export]
macro_rules! lookup_one {
    ($label:literal, $group:ident, $map:ty, $rng:ident, $iter:ident, $key:ty) => {
        $group.bench_with_input($label, &$rng, |b, rng| {
            const SETTINGS: $crate::Settings = $crate::$iter;
            let mut rng = rng.clone();
            let mut map = $crate::setup!($map, rng, $iter, $key);
            b.iter(|| {
                for i in 0..SETTINGS.count {
                    let key = <$key>::rand(&mut rng, SETTINGS.range);
                    black_box(map.get(&key));
                }
            })
        });
    };
}

#[macro_export]
macro_rules! lookup {
    ($group:ident, $map:ty, $rng:ident) => {
        $crate::lookup_one!("lookup_small_few", $group, $map, $rng, SMALL_COUNT, SmallKey);
        $crate::lookup_one!("lookup_small_many", $group, $map, $rng, LARGE_COUNT, SmallKey);
        $crate::lookup_one!("lookup_large_few", $group, $map, $rng, SMALL_COUNT, LargeKey);
        $crate::lookup_one!("insert_large_many", $group, $map, $rng, LARGE_COUNT, LargeKey);
    };
}

#[macro_export]
macro_rules! bench {
    ($map:ty) => {
        use $crate::*;

        fn bench_insertions(c: &mut Criterion) {
            let mut group = c.benchmark_group("insertion");

            let mut rng = Rng::with_seed(0xBAB0);
            
            $crate::insertion!(group, $map, rng);
        
            group.finish();
        }

        fn bench_lookups(c: &mut Criterion) {
            let mut group = c.benchmark_group("lookup");

            let mut rng = Rng::with_seed(0xBAB0);
            
            $crate::lookup!(group, $map, rng);
        
            group.finish();
        }

        criterion_group!(benches, bench_insertions, bench_lookups);
        criterion_main!(benches);
    };
}

/*
fn bench_iterations(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterations");

    // Benchmark full map iterations
    group.bench_function("small_map_small_key", |b| {
        let map = setup_small_map_small_key();
        b.iter(|| {
            for item in map.iter() {
                black_box(item);
            }
        })
    });

    group.bench_function("large_map_small_key", |b| {
        let map = setup_large_map_small_key();
        b.iter(|| {
            for item in map.iter() {
                black_box(item);
            }
        })
    });

    group.bench_function("small_map_large_key", |b| {
        let map = setup_small_map_large_key();
        b.iter(|| {
            for item in map.iter() {
                black_box(item);
            }
        })
    });

    group.bench_function("large_map_large_key", |b| {
        let map = setup_large_map_large_key();
        b.iter(|| {
            for item in map.iter() {
                black_box(item);
            }
        })
    });

    group.finish();
}
*/