//! Wizard step 3 — Store Catalog.
//!
//! Clones the official FreeSynergy Store Git repository to the local node so
//! that subsequent steps (Bundle selection, Install) can read the catalog
//! without requiring network access.
//!
//! If the store is already present, the clone is skipped.  A failed clone is
//! non-fatal: the wizard continues with the built-in bundle defaults.

use crate::error::FsInitError;
use crate::keys;
use crate::store_clone;
use crate::wizard::{StepResult, WizardState, WizardStep};

pub struct StoreLoadStep;

impl WizardStep for StoreLoadStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_STORE_LOAD_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        println!();
        let target = store_clone::default_store_dir();

        if target.exists() {
            println!("{}", keys::INIT_STEP_STORE_LOAD_EXISTS);
        } else {
            println!("{}", keys::INIT_STEP_STORE_LOAD_CLONING);
            match store_clone::clone_store(
                store_clone::DEFAULT_STORE_URL,
                store_clone::DEFAULT_BRANCH,
                &target,
            ) {
                Ok(()) => {
                    println!("{}", keys::INIT_STEP_STORE_LOAD_DONE);
                }
                Err(e) => {
                    // Non-fatal — warn and continue with built-in defaults.
                    println!("{}", keys::INIT_STEP_STORE_LOAD_FAILED);
                    println!("  ({e})");
                }
            }
        }

        // Store the resolved path in wizard state so subsequent steps can use it.
        state.store_dir = target;

        println!();
        Ok(StepResult::Next)
    }
}
