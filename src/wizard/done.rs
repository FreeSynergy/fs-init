//! Wizard step 8 — Done.
//!
//! Shows a completion summary and hints at the next steps:
//! 1. Start `fs-manager` to configure the installed services.
//! 2. Open the Store to browse additional packages.
//!
//! Attempts to launch `fs-manager start` automatically if the binary is on
//! the PATH.  The launch is best-effort — a failed exec is reported but does
//! not prevent the wizard from completing successfully.

use crate::error::FsInitError;
use crate::keys;
use crate::wizard::{StepResult, WizardState, WizardStep};

pub struct DoneStep;

impl WizardStep for DoneStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_DONE_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        println!();
        println!("{}", keys::INIT_STEP_DONE_BODY);
        println!();
        print!("{}", keys::INIT_STEP_DONE_STORE_PATH);
        println!("{}", state.store_dir.display());
        println!();
        println!("{}", keys::INIT_STEP_DONE_NEXT_STEPS);
        println!("{}", keys::INIT_STEP_DONE_MANAGER_HINT);
        println!("{}", keys::INIT_STEP_DONE_STORE_HINT);
        println!();

        // Try to launch fs-manager start automatically.
        try_launch_manager();

        Ok(StepResult::Next)
    }
}

// ── Manager launch ────────────────────────────────────────────────────────────

/// Try to exec `fs-manager start`.  Non-fatal — only prints a hint on failure.
fn try_launch_manager() {
    match std::process::Command::new("fs-manager")
        .arg("start")
        .spawn()
    {
        Ok(_child) => {
            println!("  Launched 'fs-manager start' in the background.");
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Binary not yet installed — this is expected on first bootstrap.
            println!("  (fs-manager not found — run it manually once services are installed)");
        }
        Err(e) => {
            println!("  Warning: could not launch fs-manager: {e}");
        }
    }
}
