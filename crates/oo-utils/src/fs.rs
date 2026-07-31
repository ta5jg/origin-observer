// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-utils/src/fs.rs
// Purpose : Filesystem helpers that keep written evidence intact.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Filesystem helpers that keep written evidence intact.
//!
//! Snapshots, evidence records and reports are written once and then cited.
//! A partially written file that still carries the expected name would be cited
//! as if it were complete, so writes go to a temporary file in the same
//! directory and are renamed into place only after the content is fully
//! written.

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::error::{UtilsError, UtilsResult};

/// Reads a file into a string, naming the path when the read fails.
pub fn read_to_string(path: impl AsRef<Path>) -> UtilsResult<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(|error| UtilsError::filesystem(path.display().to_string(), error))
}

/// Reads a file into bytes, naming the path when the read fails.
pub fn read(path: impl AsRef<Path>) -> UtilsResult<Vec<u8>> {
    let path = path.as_ref();
    fs::read(path).map_err(|error| UtilsError::filesystem(path.display().to_string(), error))
}

/// Creates a directory and every missing parent.
pub fn ensure_directory(path: impl AsRef<Path>) -> UtilsResult<()> {
    let path = path.as_ref();
    fs::create_dir_all(path)
        .map_err(|error| UtilsError::filesystem(path.display().to_string(), error))
}

/// Writes bytes atomically: a reader sees either the previous file or the new
/// one, never a partial write.
///
/// The temporary file is created in the destination directory so the final
/// rename stays on one filesystem, where it is atomic.
pub fn write_atomic(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> UtilsResult<()> {
    let path = path.as_ref();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    ensure_directory(parent)?;

    let temporary = temporary_path(path);
    fs::write(&temporary, contents)
        .map_err(|error| UtilsError::filesystem(temporary.display().to_string(), error))?;

    match fs::rename(&temporary, path) {
        Ok(()) => Ok(()),
        Err(error) => {
            // Leaving the temporary file behind would let a later run mistake it
            // for real output.
            let _ = fs::remove_file(&temporary);
            Err(UtilsError::filesystem(path.display().to_string(), error))
        }
    }
}

/// Writes a string atomically.
pub fn write_atomic_str(path: impl AsRef<Path>, contents: impl AsRef<str>) -> UtilsResult<()> {
    write_atomic(path, contents.as_ref().as_bytes())
}

/// Returns whether a path exists and is a readable file.
#[must_use]
pub fn is_file(path: impl AsRef<Path>) -> bool {
    path.as_ref().is_file()
}

/// Lists the files directly inside a directory, sorted by path.
///
/// The order is deterministic so a run over the same directory produces the
/// same sequence on every machine; directory iteration order is not.
pub fn list_files(directory: impl AsRef<Path>) -> UtilsResult<Vec<PathBuf>> {
    let directory = directory.as_ref();
    let entries = fs::read_dir(directory)
        .map_err(|error| UtilsError::filesystem(directory.display().to_string(), error))?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry
            .map_err(|error| UtilsError::filesystem(directory.display().to_string(), error))?;
        if entry.path().is_file() {
            files.push(entry.path());
        }
    }
    files.sort();
    Ok(files)
}

fn temporary_path(path: &Path) -> PathBuf {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("output");
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    parent.join(format!(".{name}.{}.tmp", std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("oo-utils-fs-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("scratch directory");
        path
    }

    #[test]
    fn atomic_write_creates_missing_directories() {
        let root = scratch("nested");
        let target = root.join("evidence").join("record.json");
        write_atomic_str(&target, "{}").expect("write");
        assert_eq!(read_to_string(&target).expect("read"), "{}");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn atomic_write_replaces_previous_content_and_leaves_no_temporary() {
        let root = scratch("replace");
        let target = root.join("record.json");
        write_atomic_str(&target, "first").expect("first write");
        write_atomic_str(&target, "second").expect("second write");

        assert_eq!(read_to_string(&target).expect("read"), "second");
        let leftovers: Vec<_> = list_files(&root)
            .expect("list")
            .into_iter()
            .filter(|path| path.to_string_lossy().contains(".tmp"))
            .collect();
        assert!(leftovers.is_empty(), "{leftovers:?}");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn a_failed_read_names_the_path() {
        let error = read_to_string("/nonexistent/origin-observer/missing.toml").unwrap_err();
        assert!(error.to_string().contains("missing.toml"), "{error}");
    }

    #[test]
    fn listing_is_sorted_and_excludes_directories() {
        let root = scratch("listing");
        write_atomic_str(root.join("b.txt"), "b").expect("write b");
        write_atomic_str(root.join("a.txt"), "a").expect("write a");
        ensure_directory(root.join("sub")).expect("subdirectory");

        let names: Vec<String> = list_files(&root)
            .expect("list")
            .into_iter()
            .filter_map(|path| {
                path.file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .collect();
        assert_eq!(names, vec!["a.txt".to_owned(), "b.txt".to_owned()]);
        let _ = fs::remove_dir_all(&root);
    }
}
