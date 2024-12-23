bench::bench!(
    "safe_ex::HashMap" safe_ex::HashMap<_, _, ahash::RandomState>,
    "safe_iex::HashMap" safe_iex::HashMap<_, _, ahash::RandomState>,
    "unsafe_ex::HashMap" unsafe_ex::HashMap<_, _, ahash::RandomState>,
    "std::collections::HashMap" ahash::AHashMap<_, _>
);