use chrono::{DateTime, SecondsFormat, Utc};
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Mirrors the JSONized output of `<flake>.sourceInfo`
#[allow(unused)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawSourceInfo {
    last_modified: i64,
    last_modified_date: String,
    nar_hash: String,
    store_path: String,
    submodules: bool,

    // clean-tree fields
    rev: Option<String>,
    rev_count: Option<u64>,
    short_rev: Option<String>,

    // dirty-tree fields
    dirty_rev: Option<String>,
    dirty_short_rev: Option<String>,
}

/// Provides information about the source that was used for the build.
#[derive(Debug)]
pub struct SourceInfo {
    /// Last modification of the flake's source (git commit timestamp).
    pub last_modified: DateTime<Utc>,
    /// The SHA-256 (in SRI format) of the NAR serialization of the flake's source tree.
    pub nar_hash: String,
    /// The path in the Nix store of the flake's source tree.
    pub store_path: PathBuf,
    /// Whether the git fetcher was asked to fetch submodules.
    pub submodules: bool,
    /// The commit hash of the flake's repository.
    /// Can have suffix `-dirty`.
    pub rev: String,
    /// Short rev. Git decides how long it is (**probably 7 characters, NOT 8!**).
    /// Can have suffix `-dirty`.
    pub short_rev: String,
    /// The number of ancestors of the revision `rev`. Only set if not dirty.
    pub rev_count: Option<u64>,
}

impl SourceInfo {
    fn load() -> Option<Self> {
        let payload = ver_stub::custom()?;

        let raw = serde_json::from_str::<RawSourceInfo>(payload)
            .expect("source info deserialization failed");

        Some(Self {
            last_modified: DateTime::<Utc>::from_timestamp(raw.last_modified, 0)
                .expect("source info contains invalid timestamp"),
            nar_hash: raw.nar_hash,
            store_path: PathBuf::from(raw.store_path),
            submodules: raw.submodules,
            rev: raw
                .rev
                .or(raw.dirty_rev)
                .expect("`rev` is missing in source info"),
            short_rev: raw
                .short_rev
                .or(raw.dirty_short_rev)
                .expect("`short_rev` is missing in source info"),
            rev_count: raw.rev_count,
        })
    }

    pub fn get() -> Option<&'static Self> {
        static INFO: OnceLock<Option<SourceInfo>> = OnceLock::new();
        INFO.get_or_init(Self::load).as_ref()
    }

    /// Full commit rev without `-dirty` suffix.
    pub fn rev(&self) -> &str {
        self.rev.strip_suffix("-dirty").unwrap_or(&self.rev)
    }

    /// 8-char long git rev. This is different from `short_rev`, where the length is decided by git.
    pub fn rev8(&self) -> &str {
        &self.rev[..8]
    }

    /// RFC3339-formatted commit timestamp.
    pub fn timestamp(&self) -> String {
        self.last_modified
            .to_rfc3339_opts(SecondsFormat::Secs, true)
    }

    /// Checks whether the source was dirty.
    pub fn is_dirty(&self) -> bool {
        self.rev.ends_with("-dirty")
    }
}
