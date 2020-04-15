//! When you literally just want a literal of the `std::collections` types.

/// Literally just a HashMap literal with keys and values into'd.
/// ```rust
/// # use std::collections::HashMap;
/// # use literally::hmap;
/// let m: HashMap<String, String> = hmap!{ "key" => "value" };
/// assert_eq!(m.get("key"), Some(&"value".to_string()))
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! hmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hmap!(@single $rest)),*]));

    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let _cap = hmap!(@count $($key),*);
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
/// # use std::collections::HashSet;
/// # use literally::hset;
/// let s: HashSet<String> = hset!{ "value" };
/// assert_eq!(s.get("value"), Some(&"value".to_string()))
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
/// # use std::collections::BTreeMap;
/// # use literally::bmap;
/// let m: BTreeMap<String, String> = bmap!{ "key" => "value" };
/// assert_eq!(m.get("key"), Some(&"value".to_string()))
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

/// Literally just a BTreeSet literal with values into'd.
/// ```rust
/// # use std::collections::BTreeSet;
/// # use literally::bset;
/// let s: BTreeSet<String> = bset!{ "value" };
/// assert_eq!(s.get("value"), Some(&"value".to_string()))
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! bset {
    ( $($key:expr),* $(,)? ) => {
        {
            let mut _set = ::std::collections::BTreeSet::new();
            $(
                _set.insert($key.into());
            )*
            _set
        }
    };
}

/// Literally just a VecDeque literal with values into'd.
/// ```rust
/// # use std::collections::VecDeque;
/// # use literally::vecd;
/// let s: VecDeque<String> = vecd![ "value" ];
/// assert_eq!(s.get(0), Some(&"value".to_string()))
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! vecd {
    ( $($key:expr),* $(,)? ) => {
        ::std::collections::VecDeque::from(::std::vec![$(
            $key.into()
        ),*])
    }
}

/// Literally just a LinkedList literal with values into'd.
/// ```rust
/// # use std::collections::LinkedList;
/// # use literally::list;
/// let l: LinkedList<String> = list![ "value" ];
/// assert_eq!(l.front(), Some(&"value".to_string()))
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! list {
    ( $($key:expr),* $(,)? ) => {
        {
            let mut _lst = ::std::collections::LinkedList::new();
            $(
                _lst.push_back($key.into());
            )*
            _lst
        }
    };
}

/// Literally just a BinaryHeap literal with values into'd.
/// ```rust
/// # use std::collections::BinaryHeap;
/// # use literally::heap;
/// let l: BinaryHeap<String> = heap![ "value" ];
/// assert_eq!(l.peek(), Some(&"value".to_string()))
/// ```
///
#[macro_export(local_inner_macros)]
macro_rules! heap {
    ( $($key:expr),* $(,)? ) => {
        {
            let mut _heap = ::std::collections::BinaryHeap::new();
            $(
                _heap.push($key.into());
            )*
            _heap
        }
    };
}
