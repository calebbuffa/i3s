#[allow(unused_imports)]
use super::*;
///Scanning an SLPK (ZIP store) containing millions of documents is usually inefficient and slow. A hash table file may be added to the SLPK to improve first load and file scanning performances. A hash table is a data structure that implements an associative array abstract data type, a structure that can map keys to values. A hash table uses a hash function to compute an index, also called a hash code, into an array of buckets or slots, from which the desired value can be found (Wikipedia).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlpkHashtable {}
