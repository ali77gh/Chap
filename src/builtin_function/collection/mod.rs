pub mod get; // [1 2 3 4], 1 -> get -> $int , {"a":1 "b":2} -> get -> $value
pub mod has; // [1 2 3 4], 5 -> has -> $bool, {"a":1 "b":2} -> has -> $bool
pub mod insert; // [1 2 3 4], 5 -> insert, {"a":1 "b":2}, {"c":3} -> insert
pub mod remove; // [1 2 3 4], 2 -> remove, {"a":1, "b":2}, "b" -> remove
