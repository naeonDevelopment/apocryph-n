//! Single-writer local persistence. Trusted private directory and cooperative local writers only.
//! FNV-1a detects some accidental corruption; it is NOT cryptographic tamper evidence.
use std::{fs::{self, File, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}};
use crate::{commerce::*, commerce_wire::{self, MAX_RECORD}};
pub const MAX_LOG: usize = 64 * 1024 * 1024;
fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325u64, |v, b| (v ^ *b as u64).wrapping_mul(0x100000001b3))
}
fn frame(sequence: usize, text: &str) -> Vec<u8> {
    format!("J1\t{}\t{}\t{:016x}\n{}\n", sequence, text.len(), checksum(text.as_bytes()), text).into_bytes()
}
/// Strict bounded replay: never repairs or silently discards a trailing fragment.
pub fn replay(bytes: &[u8]) -> Result<Commerce> {
    if bytes.len() > MAX_LOG { return Err(Error::Limit); }
    let mut state = Commerce::default(); let mut offset = 0;
    while offset < bytes.len() {
        let rest = &bytes[offset..];
        let header_end = rest.iter().take(96).position(|b| *b == b'\n').ok_or(Error::Corrupt)?;
        let header = std::str::from_utf8(&rest[..header_end]).map_err(|_| Error::Corrupt)?;
        let f: Vec<_> = header.split('\t').collect();
        if f.len() != 4 || f[0] != "J1" { return Err(Error::Corrupt); }
        let len: usize = f[2].parse().map_err(|_| Error::Corrupt)?;
        if len == 0 || len > MAX_RECORD { return Err(Error::Limit); }
        let end = header_end.checked_add(1).and_then(|v| v.checked_add(len)).ok_or(Error::Limit)?;
        if end >= rest.len() || rest[end] != b'\n' { return Err(Error::Corrupt); }
        let text = std::str::from_utf8(&rest[header_end + 1..end]).map_err(|_| Error::Corrupt)?;
        let expected = frame(state.command_count() + 1, text);
        if expected.as_slice() != &rest[..end + 1] { return Err(Error::Corrupt); }
        let command = commerce_wire::decode(text)?;
        if state.apply(&command)? != Applied::New { return Err(Error::Corrupt); }
        offset += end + 1;
    }
    Ok(state)
}
/// A trusted storage boundary: Ok means all bytes appended AND synchronized.
/// Any Err is ambiguous, even if the implementation believes no bytes were written.
/// Custom implementations are for isolated fault fixtures, not a durability claim.
pub trait AppendSync { fn append_sync(&mut self, bytes: &[u8]) -> std::io::Result<()>; }
pub struct Journal<S: AppendSync> { state: Commerce, sink: S, bytes: usize, poisoned: bool }
impl<S: AppendSync> Journal<S> {
    /// Caller must bind sink to exactly these bytes under exclusive ownership.
    pub fn recover(bytes: &[u8], sink: S) -> Result<Self> {
        Ok(Self { state: replay(bytes)?, sink, bytes: bytes.len(), poisoned: false })
    }
    pub fn state(&self) -> &Commerce { &self.state }
    pub fn is_poisoned(&self) -> bool { self.poisoned }
    pub fn apply(&mut self, command: &Command) -> Result<Applied> {
        if self.poisoned { return Err(Error::Poisoned); }
        let mut next = self.state.clone();
        let result = next.apply(command)?;
        if result == Applied::Replay { return Ok(result); }
        let encoded = frame(next.command_count(), &commerce_wire::encode(command)?);
        let size = self.bytes.checked_add(encoded.len()).ok_or(Error::Limit)?;
        if size > MAX_LOG { return Err(Error::Limit); }
        if self.sink.append_sync(&encoded).is_err() {
            self.poisoned = true;
            return Err(Error::Io);
        }
        self.bytes = size; self.state = next;
        Ok(Applied::New)
    }
}
struct FileSink(File);
impl AppendSync for FileSink {
    fn append_sync(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.0.write_all(bytes)?;
        self.0.sync_all()
    }
}
pub struct FileStore { journal: Journal<FileSink>, lock: PathBuf }
impl FileStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let parent = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
        let parent = fs::canonicalize(parent).map_err(|_| Error::Io)?;
        let name = path.file_name().ok_or(Error::Invalid)?;
        let path = parent.join(name);
        let mut lock_name = name.to_os_string(); lock_name.push(".lock");
        let lock = parent.join(lock_name);
        // Never remove an existing lock, inspect PIDs, or guess that a lock is stale.
        fs::create_dir(&lock).map_err(|e| if e.kind() == std::io::ErrorKind::AlreadyExists { Error::Locked } else { Error::Io })?;
        // Any failure after acquisition leaves the lock for explicit offline reconciliation.
        File::open(&parent).and_then(|f| f.sync_all()).map_err(|_| Error::Io)?;
        match fs::symlink_metadata(&path) {
            Ok(m) if !m.file_type().is_file() => return Err(Error::Invalid),
            Ok(_) => {},
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {},
            Err(_) => return Err(Error::Io),
        }
        let mut file = OpenOptions::new().create(true).read(true).append(true).open(&path).map_err(|_| Error::Io)?;
        let metadata = file.metadata().map_err(|_| Error::Io)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if metadata.nlink() != 1 { return Err(Error::Invalid); }
        }
        if metadata.len() > MAX_LOG as u64 { return Err(Error::Limit); }
        let mut bytes = Vec::new();
        (&mut file).take(MAX_LOG as u64 + 1).read_to_end(&mut bytes).map_err(|_| Error::Io)?;
        if bytes.len() > MAX_LOG { return Err(Error::Limit); }
        file.sync_all().map_err(|_| Error::Io)?;
        File::open(&parent).and_then(|f| f.sync_all()).map_err(|_| Error::Io)?;
        let journal = Journal::recover(&bytes, FileSink(file))?;
        Ok(Self { journal, lock })
    }
    pub fn state(&self) -> &Commerce { self.journal.state() }
    pub fn apply(&mut self, c: &Command) -> Result<Applied> { self.journal.apply(c) }
    pub fn is_poisoned(&self) -> bool { self.journal.is_poisoned() }
    /// Only a deliberate clean close removes the lock. Drop/crash/panic retain it.
    /// Removal need not be durable: a resurrected lock fails closed after a crash.
    pub fn close(self) -> Result<()> {
        if self.journal.is_poisoned() { return Err(Error::Poisoned); }
        self.journal.sink.0.sync_all().map_err(|_| Error::Io)?;
        fs::remove_dir(&self.lock).map_err(|_| Error::Io)
    }
}
