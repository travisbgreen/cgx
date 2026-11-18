//! Provenance attached to every fact (GM-6.1): the evidence that justifies it.

use std::sync::Arc;

use crate::confidence::Tier;
use serde::{Deserialize, Serialize};

/// A source span: repo-relative file plus 1-based line and optional column.
/// Used both inside [`Provenance`] and on raw references emitted by frontends.
///
/// `file` is `Arc<str>` (2026-11-17, memory-amplification fix): a `Span` is
/// cloned once per destination whenever a placeholder edge is expanded into a
/// multi-candidate set (`build_cha_edge`/`build_sig_edge`/the structural-edge
/// builders in `cgx-resolve` all clone an existing prototype `Span`, never
/// construct a fresh one) — on a corpus the size of WS2003 or Linux this is
/// millions of full-path-string clones of what is, per source file, the exact
/// same bytes. `Arc<str>::clone` is a refcount bump; a plain `String::clone`
/// is a full heap allocation + memcpy of the path every time. The one-time
/// construction cost at a frontend's original `Span::new` call (11 extractor
/// crates) is unchanged — this only makes every *subsequent* clone cheap.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Repo-relative path.
    pub file: Arc<str>,
    /// 1-based line of the call site or definition.
    pub line: u32,
    /// Optional column (available when the parser/SCIP provides it).
    pub col: Option<u32>,
}

impl Span {
    pub fn new(file: impl Into<Arc<str>>, line: u32, col: Option<u32>) -> Self {
        Span {
            file: file.into(),
            line,
            col,
        }
    }
}

/// The provenance record on a node or edge (GM-6.1).
///
/// `rule` is `Arc<str>` (2026-11-17, memory-amplification fix): every call
/// site that builds an `EdgeWithProvenance` passes the IDENTICAL rule string
/// to both `EdgeRecord.rule` (denormalized for cheap filtering — see that
/// field's doc comment) and here — confirmed at all 6 construction sites in
/// `cgx-resolve` (`cha.rs`, `sig.rs`, `ifds.rs`, `link.rs` x3). Two owned
/// `String` allocations of the same few-byte constant (`"sig-compat"`,
/// `"cha-override"`, ...), per edge, at tens-of-millions-of-edges scale, is a
/// real avoidable cost. Callers now construct one `Arc<str>` and clone it
/// (refcount bump) into both fields instead of allocating twice.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Provenance {
    /// Where the fact was observed.
    pub span: Span,
    /// Name of the producing rule, e.g. `scope-ref`, `import-ref`, `name-arity`,
    /// `scip-occurrence`, `cha-override`, `heuristic-sentinel`, `macro-textual`.
    pub rule: Arc<str>,
    /// Resolution tier (0–4) used to produce the fact (GM-6.1, GM-5.2).
    pub tier: Tier,
    /// Content-addressed blob OID of the source file at index time. Links the
    /// fact to the exact file version that produced it (GM-6.1 `index_id`),
    /// enabling staleness detection.
    pub index_id: String,
}

impl Provenance {
    pub fn new(
        span: Span,
        rule: impl Into<Arc<str>>,
        tier: Tier,
        index_id: impl Into<String>,
    ) -> Self {
        Provenance {
            span,
            rule: rule.into(),
            tier,
            index_id: index_id.into(),
        }
    }
}
