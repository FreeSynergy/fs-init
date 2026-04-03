//! Install wizard (State Machine Pattern).
//!
//! Steps: Welcome → Capability → StoreLoad → Engine → Bundle → Confirm → Progress → Done
//!
//! All steps use plain `println!` + stdin because the render engine has not
//! been installed yet. `WizardStep` is a trait; each step is its own struct.

pub mod bundle;
pub mod capability_step;
pub mod confirm;
pub mod done;
pub mod engine;
pub mod progress;
pub mod store_load;
pub mod welcome;

use std::path::PathBuf;

use crate::capability::BootstrapCapability;
use crate::error::FsInitError;

// ── Data types shared across steps ───────────────────────────────────────────

/// A bundle the user can choose to install.
#[derive(Debug, Clone)]
pub struct BundleChoice {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requires_display: bool,
}

/// A render engine the user can choose.
#[derive(Debug, Clone)]
pub struct EngineChoice {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requires_display: bool,
}

/// Install target (package format).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallTarget {
    Container,
    Rpm,
    Deb,
    AppImage,
}

impl InstallTarget {
    pub fn label(self) -> &'static str {
        match self {
            InstallTarget::Container => crate::keys::INIT_TARGET_CONTAINER,
            InstallTarget::Rpm => crate::keys::INIT_TARGET_RPM,
            InstallTarget::Deb => crate::keys::INIT_TARGET_DEB,
            InstallTarget::AppImage => crate::keys::INIT_TARGET_APPIMAGE,
        }
    }
}

// ── Built-in bundle defaults (used when store catalog is not yet available) ───

pub fn default_bundles() -> Vec<BundleChoice> {
    vec![
        BundleChoice {
            id: "freeSynergy.bundle.minimal".to_string(),
            name: "FreeSynergy Minimal".to_string(),
            description: "Node + Registry + SQLite. For embedded systems and CI.".to_string(),
            requires_display: false,
        },
        BundleChoice {
            id: "freeSynergy.bundle.server".to_string(),
            name: "FreeSynergy Server".to_string(),
            description: "Minimal + Auth (Kanidm) + Inventory + Session. Full server stack."
                .to_string(),
            requires_display: false,
        },
        BundleChoice {
            id: "freeSynergy.bundle.workstation".to_string(),
            name: "FreeSynergy Workstation".to_string(),
            description: "Server + Desktop + Managers + Apps. Daily-driver desktop.".to_string(),
            requires_display: true,
        },
        BundleChoice {
            id: "freeSynergy.bundle.developer".to_string(),
            name: "FreeSynergy Developer".to_string(),
            description: "Workstation + Forgejo + extended developer tools.".to_string(),
            requires_display: true,
        },
    ]
}

// ── Built-in engine defaults ──────────────────────────────────────────────────

pub fn default_engines() -> Vec<EngineChoice> {
    vec![
        EngineChoice {
            id: "iced".to_string(),
            name: "iced (libcosmic)".to_string(),
            description: "Native GPU-accelerated GUI. Recommended for desktops.".to_string(),
            requires_display: true,
        },
        EngineChoice {
            id: "bevy".to_string(),
            name: "Bevy".to_string(),
            description: "3D-capable game-engine renderer. Experimental.".to_string(),
            requires_display: true,
        },
        EngineChoice {
            id: "tui".to_string(),
            name: "TUI (ratatui)".to_string(),
            description: "Terminal UI. Works without a display server.".to_string(),
            requires_display: false,
        },
        EngineChoice {
            id: "none".to_string(),
            name: "No UI (API + CLI only)".to_string(),
            description: "Headless operation via gRPC / REST only.".to_string(),
            requires_display: false,
        },
    ]
}

// ── Wizard state ──────────────────────────────────────────────────────────────

/// Accumulated choices across all wizard steps.
pub struct WizardState {
    pub capability: BootstrapCapability,
    pub post_install_hint: &'static str,
    pub selected_bundle: Option<BundleChoice>,
    pub selected_engine: Option<EngineChoice>,
    pub install_target: InstallTarget,
    /// Path to the cloned store catalog (set by StoreLoadStep).
    pub store_dir: PathBuf,
}

/// The final result handed back to the strategy after the wizard completes.
pub struct WizardResult {
    pub store_path: PathBuf,
    pub bundle_id: String,
    pub engine_id: String,
    pub install_target: InstallTarget,
}

// ── Step protocol ─────────────────────────────────────────────────────────────

/// What the wizard machine does after a step returns.
pub enum StepResult {
    /// Advance to the next step.
    Next,
    /// Go back to the previous step.
    Back,
    /// The user aborted the wizard.
    Abort,
}

/// A single step in the install wizard.
pub trait WizardStep {
    /// Short title shown as a header.
    fn title(&self) -> &'static str;
    /// Execute this step, mutating `state`. Returns how to proceed.
    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError>;
}

// ── Wizard machine ────────────────────────────────────────────────────────────

/// Drives the wizard through its steps in order, supporting back-navigation.
pub struct WizardMachine {
    steps: Vec<Box<dyn WizardStep>>,
    state: WizardState,
}

impl WizardMachine {
    /// Build the machine with all steps in sequence.
    pub fn new(cap: BootstrapCapability, post_install_hint: &'static str) -> Self {
        let install_target = default_target(&cap);
        let state = WizardState {
            capability: cap,
            post_install_hint,
            selected_bundle: None,
            selected_engine: None,
            install_target,
            store_dir: crate::store_clone::default_store_dir(),
        };
        let steps: Vec<Box<dyn WizardStep>> = vec![
            Box::new(welcome::WelcomeStep),
            Box::new(capability_step::CapabilityStep),
            Box::new(store_load::StoreLoadStep),
            Box::new(engine::EngineStep),
            Box::new(bundle::BundleStep),
            Box::new(confirm::ConfirmStep),
            Box::new(progress::ProgressStep),
            Box::new(done::DoneStep),
        ];
        WizardMachine { steps, state }
    }

    /// Run all steps to completion and return the final result.
    pub fn run(mut self) -> Result<WizardResult, FsInitError> {
        let mut idx: usize = 0;
        while idx < self.steps.len() {
            print_step_header(idx + 1, self.steps.len(), self.steps[idx].title());
            match self.steps[idx].run(&mut self.state)? {
                StepResult::Next => idx += 1,
                StepResult::Back => {
                    idx = idx.saturating_sub(1);
                }
                StepResult::Abort => return Err(FsInitError::Aborted),
            }
        }
        Ok(build_result(&self.state))
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn default_target(cap: &BootstrapCapability) -> InstallTarget {
    use crate::capability::ContainerRuntime;
    match cap.container {
        ContainerRuntime::Podman | ContainerRuntime::Docker => InstallTarget::Container,
        ContainerRuntime::None => detect_package_manager(),
    }
}

fn detect_package_manager() -> InstallTarget {
    let rpm_indicator = std::path::Path::new("/etc/redhat-release").exists()
        || std::path::Path::new("/etc/fedora-release").exists();
    let deb_indicator = std::path::Path::new("/etc/debian_version").exists();
    if rpm_indicator {
        InstallTarget::Rpm
    } else if deb_indicator {
        InstallTarget::Deb
    } else {
        InstallTarget::AppImage
    }
}

fn print_step_header(current: usize, total: usize, title: &str) {
    println!();
    println!("{}", crate::keys::INIT_DIVIDER);
    println!("  Step {current}/{total} — {title}");
    println!("{}", crate::keys::INIT_DIVIDER);
}

fn build_result(state: &WizardState) -> WizardResult {
    let bundle_id = state
        .selected_bundle
        .as_ref()
        .map_or("none", |b| b.id.as_str())
        .to_owned();
    let engine_id = state
        .selected_engine
        .as_ref()
        .map_or("none", |e| e.id.as_str())
        .to_owned();
    WizardResult {
        store_path: state.store_dir.clone(),
        bundle_id,
        engine_id,
        install_target: state.install_target,
    }
}

// ── Shared I/O helpers (used by steps) ───────────────────────────────────────

/// Read a trimmed line from stdin. Returns `None` on EOF.
pub fn read_line() -> Result<Option<String>, FsInitError> {
    let mut buf = String::new();
    let n = std::io::stdin().read_line(&mut buf)?;
    if n == 0 {
        Ok(None)
    } else {
        Ok(Some(buf.trim().to_owned()))
    }
}

/// Prompt the user and return the trimmed input.
pub fn prompt(msg: &str) -> Result<String, FsInitError> {
    use std::io::Write;
    print!("{msg}");
    std::io::stdout().flush()?;
    Ok(read_line()?.unwrap_or_default())
}
