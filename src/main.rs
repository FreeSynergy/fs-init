#![deny(clippy::all, clippy::pedantic, warnings)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::doc_markdown)]
//! `fs-init` — FreeSynergy bootstrap binary.
//!
//! # Design
//!
//! Uses a **Strategy Pattern** to select the bootstrap mode (GUI / TUI / Headless)
//! based on the system capabilities detected by `fs-info`.
//!
//! Uses a **State Machine** (wizard) to guide the user through:
//! `Welcome → Capability → StoreLoad → Engine → Bundle → Confirm → Progress → Done`
//!
//! All output before the render engine is installed is plain `println!`.
//! Every user-facing string is a constant defined in `keys.rs` (English fallback).
//!
//! # Phase 1 scope
//!
//! - Capability detection via `fs-info`
//! - Strategy selection (Gui / Tui / Headless)
//! - Install wizard (text-based, stdin/stdout)
//! - Store catalog clone (via `gix`)
//! - Bundle install via `fs-store` Pipeline
//! - Manager startup hint after install

mod capability;
mod catalog_reader;
mod error;
mod keys;
mod store_clone;
mod strategy;
mod wizard;

use clap::Parser;
use std::path::PathBuf;

// ── CLI ───────────────────────────────────────────────────────────────────────

/// `FreeSynergy` Init — bootstrap a node by detecting capabilities and
/// cloning the official store.
#[derive(Parser, Debug)]
#[command(name = "fs-init", about = "Bootstrap FreeSynergy on this node")]
struct Args {
    /// Store Git repository URL to clone.
    #[arg(long, default_value = store_clone::DEFAULT_STORE_URL)]
    store_url: String,

    /// Directory to clone the store into.
    #[arg(long)]
    target_dir: Option<PathBuf>,

    /// Git branch to check out.
    #[arg(long, default_value = store_clone::DEFAULT_BRANCH)]
    branch: String,

    /// Skip the interactive wizard and just clone the store.
    #[arg(long)]
    clone_only: bool,
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{}", keys::INIT_TITLE);
    println!("{}", keys::INIT_DIVIDER);

    if args.clone_only {
        run_clone_only(&args);
        return;
    }

    println!("{}", keys::INIT_DETECTING_CAPABILITIES);
    let cap = capability::BootstrapCapability::detect();

    if let Err(e) = strategy::run(cap) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

// ── Clone-only mode (non-interactive) ────────────────────────────────────────

fn run_clone_only(args: &Args) {
    let target = args
        .target_dir
        .clone()
        .unwrap_or_else(store_clone::default_store_dir);

    if target.exists() {
        println!("{}", keys::INIT_STEP_PROGRESS_CLONE_EXISTS);
        println!("  Path: {}", target.display());
        return;
    }

    println!("{}", keys::INIT_STEP_PROGRESS_CLONING_STORE);
    println!("  {} → {}", args.store_url, target.display());

    if let Err(e) = store_clone::clone_store(&args.store_url, &args.branch, &target) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    println!("{}", keys::INIT_STEP_PROGRESS_CLONE_OK);
    println!("  Path: {}", target.display());
}
