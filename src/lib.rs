//! For when you just want a literal.

/// Literally just a HashMap literal with keys and values into'd.
/// ```rust
/// let m = HashMap<String, String> = hmap!{ "key" => "value" };
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! hmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hmap!(@single $rest)),*]));

    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let _cap = map!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key.into(), $value.into());
            )*
            _map
        }
    };
}

/// Literally just a HashSet literal with values into'd.
/// ```rust
/// let s = HashSet<String> = hset!{ "value" };
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! hset {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hset!(@single $rest)),*]));

    ($($key:expr,)+) => { hset!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = hset!(@count $($key),*);
            let mut _set = ::std::collections::HashSet::with_capacity(_cap);
            $(
                let _ = _set.insert($key.into());
            )*
            _set
        }
    };
}

/// Literally just a BTreeMap literal with keys and values into'd.
/// ```rust
/// let m = BTreeMap<String, String> = bmap!{ "key" => "value" };
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! bmap {
    ( $($key:expr => $value:expr),* $(,)?) => {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                let _ = _map.insert($key.into(), $value.into());
            )*
            _map
        }
    };
}

/// Literally just a HashSet literal with values into'd.
/// ```rust
/// let s = BTreeSet<String> = bset!{ "value" };
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! bset {
    ($($key:expr,)+) => (btreeset!($($key),+));

    ( $($key:expr),* ) => {
        {
            let mut _set = ::std::collections::BTreeSet::new();
            $(
                _set.insert($key.into());
            )*
            _set
        }
    };
}
