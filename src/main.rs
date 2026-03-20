// Minimal bootstrap binary: clones the FSN official store repository
// to a local path so the node can use packages offline.
//
// Usage:
//   fsn-init                          → clones to ~/.local/share/fsn/store
//   fsn-init --store-url <url>        → use a custom store URL
//   fsn-init --target-dir <path>      → clone to a custom directory
//   fsn-init --branch <branch>        → clone a specific branch

use std::path::PathBuf;
use clap::Parser;

const DEFAULT_STORE_URL: &str = "https://github.com/FreeSynergy/Store.git";

/// FreeSynergy Init — bootstraps a node by cloning the official store.
#[derive(Parser, Debug)]
#[command(name = "fsn-init", about = "Bootstrap FreeSynergy by cloning the store")]
struct Args {
    /// Store Git repository URL to clone.
    #[arg(long, default_value = DEFAULT_STORE_URL)]
    store_url: String,

    /// Directory to clone the store into.
    #[arg(long)]
    target_dir: Option<PathBuf>,

    /// Git branch to check out.
    #[arg(long, default_value = "main")]
    branch: String,
}

fn main() {
    let args = Args::parse();

    let target = args.target_dir.unwrap_or_else(default_store_dir);

    if target.exists() {
        eprintln!("Store already exists at {}", target.display());
        std::process::exit(0);
    }

    println!("Cloning {} → {}", args.store_url, target.display());

    if let Err(e) = clone_store(&args.store_url, &args.branch, &target) {
        eprintln!("Clone failed: {e}");
        std::process::exit(1);
    }

    println!("Done. Store ready at {}", target.display());
}

fn default_store_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".local/share/fsn/store")
}

fn clone_store(url: &str, branch: &str, target: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut prepare = gix::clone::PrepareFetch::new(
        url,
        target,
        gix::create::Kind::WithWorktree,
        gix::create::Options::default(),
        gix::open::Options::isolated(),
    )?;
    prepare = prepare.with_remote_name("origin")?;

    // Configure refspec so gix fetches (and checks out) the requested branch.
    // For the default branch we rely on the remote HEAD; for others we set an
    // explicit mapping so the working tree lands on the right branch.
    let branch = branch.to_owned();
    prepare = prepare.configure_remote(move |remote| {
        let spec = format!("+refs/heads/{branch}:refs/remotes/origin/{branch}");
        remote.with_refspecs([spec.as_str()], gix::remote::Direction::Fetch)
    })?;

    let (mut checkout, _outcome) = prepare.fetch_then_checkout(
        gix::progress::Discard,
        &gix::interrupt::IS_INTERRUPTED,
    )?;

    let (_repo, _outcome) = checkout.main_worktree(
        gix::progress::Discard,
        &gix::interrupt::IS_INTERRUPTED,
    )?;

    Ok(())
}
