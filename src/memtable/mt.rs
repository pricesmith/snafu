/// MemTable holds a SORTED list of the latest written records. 
/// 
/// Writes are duplicated to to the WAL for Memtable recovery in the event of a restart.
/// 
/// Memtables have a max capacity, which, when reached we flush it to disk as a Table(SSTable).
/// 
/// Entries are stored in a Vector instead of a Hashmap to support Scans.
pub struct MemTable {
    entries: Vec<MemTableEntry>,
    size: usize,
}

/// MemTableEntry
pub struct MemTableEntry {
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
    pub timestamp: u128,
    pub deleted: bool,
}

/// new() creates a new, empty MemTable.
pub fn new() -> MemTable {
    MemTable {
        entries: Vec::new(),
        size: 0,
    }
}

/// Performs Binary Search to find a record in the MemTable.
///
/// If the record is found `[Result::Ok]` is returned, with the index of record. If the record is not
/// found then `[Result::Err]` is returned, with the index to insert the record at.
fn get_index(&self, key: &[u8]) -> Result<usize, usize> {
    self
        .entries
        .binary_search_by_key(&key, |e| e.key.as_slice())
}