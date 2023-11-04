
// index starts from 1
pub mod insert;        // [1 2 3 4], 5 -> insert
pub mod get;           // [1 2 3 4], 1 -> get -> $int
pub mod pop;           // [1 2 3 4] -> pop -> $int
pub mod last;           // [1 2 3 4] -> last -> $int
//pub mod len;           // string function
pub mod has;           // [1 2 3 4], 5 -> has -> $bool
pub mod remove;        // [1 2 3 4], 2 -> remove
pub mod remove_at;     // [1 2 3 4], 2 -> remove_at
pub mod index_of;     // [1 2 3 4], 2 -> remove_at