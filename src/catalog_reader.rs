//! Minimal TOML reader for the local FreeSynergy Store catalog.
//!
//! Reads bundle definitions from the cloned store without loading the full
//! fs-store Inventory. Used by the wizard's Bundle step and Install step.
//!
//! Design: plain `serde` + `toml` — no async, no HTTP. The catalog is already
//! on disk when this is called (cloned by `StoreLoadStep`).

use std::path::{Path, PathBuf};

use serde::Deserialize;

// ── Public domain types ───────────────────────────────────────────────────────

/// A bundle read from the local store catalog.
#[derive(Debug, Clone)]
pub struct CatalogBundle {
    /// Package id, e.g. `"freeSynergy.bundle.minimal"`.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// One-line description.
    pub description: String,
    /// True if this bundle requires a display server (has a desktop).
    pub requires_display: bool,
    /// Required components (optional ones are excluded for Phase 1).
    pub components: Vec<CatalogComponent>,
}

/// One required component within a bundle.
#[derive(Debug, Clone)]
pub struct CatalogComponent {
    /// Component id, e.g. `"node"`.
    pub id: String,
    /// Package type from the component's own catalog, e.g. `"container"`.
    pub package_type: String,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Load all bundles from `{store_dir}/packages/bundles/`.
///
/// Skips any bundle or component that fails to parse — partial results are
/// better than no results.  Returns an empty vec if the namespace index is
/// missing.
#[must_use]
pub fn load_bundles(store_dir: &Path) -> Vec<CatalogBundle> {
    let bundles_dir = store_dir.join("packages/bundles");
    let index_path = bundles_dir.join("catalog.toml");

    let Ok(raw) = std::fs::read_to_string(&index_path) else {
        tracing::warn!("bundle namespace index not found: {}", index_path.display());
        return vec![];
    };

    let Ok(index) = toml::from_str::<NamespaceIndex>(&raw) else {
        tracing::warn!("failed to parse bundle namespace index");
        return vec![];
    };

    index
        .packages
        .iter()
        .filter_map(|pkg_ref| {
            let bundle_catalog_path = bundles_dir.join(&pkg_ref.catalog);
            load_bundle(&bundle_catalog_path, &bundles_dir, store_dir)
        })
        .collect()
}

// ── Private TOML shapes ───────────────────────────────────────────────────────

/// `packages/{namespace}/catalog.toml` — namespace index.
#[derive(Debug, Deserialize)]
struct NamespaceIndex {
    #[serde(default)]
    packages: Vec<PackageRef>,
}

#[derive(Debug, Deserialize)]
struct PackageRef {
    #[allow(dead_code)]
    id: String,
    catalog: String,
}

/// Individual `catalog.toml` — minimal fields only.
#[derive(Debug, Deserialize)]
struct RawEntry {
    package: RawMeta,
    #[serde(default)]
    bundle: Option<RawBundle>,
}

#[derive(Debug, Deserialize)]
struct RawMeta {
    id: String,
    name: String,
    summary: String,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawBundle {
    #[serde(default)]
    components: Vec<RawComponent>,
}

#[derive(Debug, Deserialize)]
struct RawComponent {
    id: String,
    #[serde(default)]
    catalog: Option<String>,
    #[serde(default)]
    optional: bool,
    /// Placeholder for the engine-choice pseudo-component.
    #[serde(default)]
    engine_choice: bool,
}

/// Minimal shape for a component's own `catalog.toml`.
#[derive(Debug, Deserialize)]
struct RawComponentEntry {
    package: RawComponentMeta,
}

#[derive(Debug, Deserialize)]
struct RawComponentMeta {
    #[serde(rename = "type")]
    package_type: String,
}

// ── Private helpers ───────────────────────────────────────────────────────────

fn load_bundle(catalog_path: &Path, bundle_dir: &Path, store_dir: &Path) -> Option<CatalogBundle> {
    let raw = std::fs::read_to_string(catalog_path)
        .map_err(|e| tracing::warn!("could not read {}: {e}", catalog_path.display()))
        .ok()?;

    let entry: RawEntry = toml::from_str(&raw)
        .map_err(|e| tracing::warn!("could not parse {}: {e}", catalog_path.display()))
        .ok()?;

    let raw_bundle = entry.bundle.unwrap_or(RawBundle { components: vec![] });

    let requires_display = entry
        .package
        .tags
        .iter()
        .any(|t| t == "desktop" || t == "workstation");

    let components = raw_bundle
        .components
        .iter()
        .filter(|c| !c.optional && !c.engine_choice)
        .map(|c| resolve_component(c, catalog_path, bundle_dir, store_dir))
        .collect();

    Some(CatalogBundle {
        id: entry.package.id,
        name: entry.package.name,
        description: entry.package.summary,
        requires_display,
        components,
    })
}

fn resolve_component(
    comp: &RawComponent,
    bundle_catalog_path: &Path,
    _bundle_dir: &Path,
    store_dir: &Path,
) -> CatalogComponent {
    // Resolve relative catalog path from the bundle catalog file's directory.
    let pkg_type = comp
        .catalog
        .as_deref()
        .and_then(|rel_catalog| {
            let bundle_file_dir = bundle_catalog_path.parent()?;
            let comp_catalog = bundle_file_dir.join(rel_catalog);
            let comp_catalog = normalize_path(&comp_catalog);
            read_component_type(&comp_catalog)
        })
        .or_else(|| guess_type_by_id(&comp.id, store_dir))
        .unwrap_or_else(|| "other".to_string());

    CatalogComponent {
        id: comp.id.clone(),
        package_type: pkg_type,
    }
}

/// Try to read `[package].type` from a component catalog file.
fn read_component_type(catalog_path: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(catalog_path).ok()?;
    let entry: RawComponentEntry = toml::from_str(&raw).ok()?;
    Some(entry.package.package_type)
}

/// Heuristic fallback: guess package type from the component id.
fn guess_type_by_id(id: &str, store_dir: &Path) -> Option<String> {
    // Check common catalog locations by id.
    let candidates = [
        format!("packages/apps/{id}/catalog.toml"),
        format!("packages/containers/{id}/catalog.toml"),
        format!("packages/adapters/{id}/catalog.toml"),
        format!("packages/bundles/{id}/catalog.toml"),
    ];
    for rel in &candidates {
        let path = store_dir.join(rel);
        if let Some(t) = read_component_type(&path) {
            return Some(t);
        }
    }
    None
}

/// Resolve `..` and `.` segments in a path without requiring the path to exist.
fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for part in path.components() {
        match part {
            std::path::Component::ParentDir => {
                result.pop();
            }
            std::path::Component::CurDir => {}
            other => result.push(other),
        }
    }
    result
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_path_resolves_parent() {
        let p = PathBuf::from("/a/b/../c/./d");
        assert_eq!(normalize_path(&p), PathBuf::from("/a/c/d"));
    }

    #[test]
    fn load_bundles_empty_on_missing_store() {
        let bundles = load_bundles(Path::new("/nonexistent/store"));
        assert!(bundles.is_empty());
    }

    #[test]
    fn load_bundles_from_local_store() {
        // Reads the actual local Store/ checkout if present.
        let store_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("Store");

        if !store_dir.exists() {
            return; // Skip if Store/ not present in CI
        }

        let bundles = load_bundles(&store_dir);
        assert!(
            !bundles.is_empty(),
            "expected at least one bundle in Store/"
        );
        for b in &bundles {
            assert!(!b.id.is_empty(), "bundle id must not be empty");
            assert!(!b.name.is_empty(), "bundle name must not be empty");
        }
    }
}
