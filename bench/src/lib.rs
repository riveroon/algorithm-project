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
    count: 1_000_000,
    range: 1_600_000
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
macro_rules! insertion_group {
    ($label:literal, $c:ident, $rng:ident, $key:ty, $iter:ident, $($name:literal $map:ty),+) => {
        {
            let mut group = $c.benchmark_group($label);
            $(
                let rng = $rng.clone();
                $crate::insert_one!($name, group, $map, rng, $iter, $key);
            )*
            group.finish();
        }
    };
}

#[macro_export]
macro_rules! insertion {
    ($c:ident, $rng:ident, $($name:literal $map:ty),+) => {
        insertion_group!("insertion_small_few", $c, $rng, SmallKey, SMALL_COUNT, $($name $map),+);
        insertion_group!("insertion_small_many", $c, $rng, SmallKey, LARGE_COUNT, $($name $map),+);
        insertion_group!("insertion_large_few", $c, $rng, LargeKey, SMALL_COUNT, $($name $map),+);
        insertion_group!("insertion_large_many", $c, $rng, LargeKey, LARGE_COUNT, $($name $map),+);
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
macro_rules! lookup_group {
    ($label:literal, $c:ident, $rng:ident, $key:ty, $iter:ident, $($name:literal $map:ty),+) => {
        {
            let mut group = $c.benchmark_group($label);
            $(
                let rng = $rng.clone();
                $crate::lookup_one!($name, group, $map, rng, $iter, $key);
            )*
            group.finish();
        }
    };
}

#[macro_export]
macro_rules! lookup {
    ($c:ident, $rng:ident, $($name:literal $map:ty),+) => {
        lookup_group!("lookup_small_few", $c, $rng, SmallKey, SMALL_COUNT, $($name $map),+);
        lookup_group!("lookup_small_many", $c, $rng, SmallKey, LARGE_COUNT, $($name $map),+);
        lookup_group!("lookup_large_few", $c, $rng, LargeKey, SMALL_COUNT, $($name $map),+);
        lookup_group!("lookup_large_many", $c, $rng, LargeKey, LARGE_COUNT, $($name $map),+);
    };
}

#[macro_export]
macro_rules! remove_one {
    ($label:literal, $group:ident, $map:ty, $rng:ident, $iter:ident, $key:ty) => {
        $group.bench_with_input($label, &$rng, |b, rng| {
            const SETTINGS: $crate::Settings = $crate::$iter;
            let mut rng = rng.clone();
            let mut map = $crate::setup!($map, rng, $iter, $key);
            b.iter(|| {
                for i in 0..SETTINGS.count {
                    let key = <$key>::rand(&mut rng, SETTINGS.range);
                    black_box(map.remove(&key));
                }
            })
        });
    };
}

#[macro_export]
macro_rules! remove_group {
    ($label:literal, $c:ident, $rng:ident, $key:ty, $iter:ident, $($name:literal $map:ty),+) => {
        {
            let mut group = $c.benchmark_group($label);
            //group.measurement_time(Duration::from_secs(10));
            $(
                let rng = $rng.clone();
                $crate::remove_one!($name, group, $map, rng, $iter, $key);
            )*
            group.finish();
        }
    };
}

#[macro_export]
macro_rules! remove {
    ($c:ident, $rng:ident, $($name:literal $map:ty),+) => {
        remove_group!("remove_small_few", $c, $rng, SmallKey, SMALL_COUNT, $($name $map),+);
        remove_group!("remove_small_many", $c, $rng, SmallKey, LARGE_COUNT, $($name $map),+);
        remove_group!("remove_large_few", $c, $rng, LargeKey, SMALL_COUNT, $($name $map),+);
        remove_group!("remove_large_many", $c, $rng, LargeKey, LARGE_COUNT, $($name $map),+);
    };
}

#[macro_export]
macro_rules! bench {
    ($($label:literal $map:ty),+) => {
        use $crate::*;
        use std::time::Duration;

        fn bench_insert(c: &mut Criterion) {
            let mut rng = Rng::with_seed(0xBAB0);
            
            $crate::insertion!(c, rng, $($label $map),*);
        }

        fn bench_lookup(c: &mut Criterion) {
            let mut rng = Rng::with_seed(0xBAB0);
            
            $crate::lookup!(c, rng, $($label $map),*);
        }

        fn bench_remove(c: &mut Criterion) {
            let mut rng = Rng::with_seed(0xBAB0);
            
            $crate::remove!(c, rng, $($label $map),*);
        }

        criterion_group!(benches, bench_insert, bench_lookup, bench_remove);
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