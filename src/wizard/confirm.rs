//! Wizard step 5 — Confirmation.

use crate::error::FsInitError;
use crate::keys;
use crate::wizard::{prompt, StepResult, WizardState, WizardStep};

pub struct ConfirmStep;

impl WizardStep for ConfirmStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_CONFIRM_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        println!();
        println!("  The following will be installed:");
        println!();

        let bundle_name = state
            .selected_bundle
            .as_ref()
            .map_or("none", |b| b.name.as_str());
        println!("{}{}", keys::INIT_STEP_CONFIRM_BUNDLE, bundle_name);

        let engine_name = state
            .selected_engine
            .as_ref()
            .map_or("none", |e| e.name.as_str());
        println!("{}{}", keys::INIT_STEP_CONFIRM_ENGINE, engine_name);

        println!(
            "{}{}",
            keys::INIT_STEP_CONFIRM_TARGET,
            state.install_target.label()
        );
        println!();
        println!("  {}", state.post_install_hint);
        println!();

        loop {
            let input = prompt(keys::INIT_STEP_CONFIRM_QUESTION)?;
            match input.to_lowercase().as_str() {
                "y" | "yes" => return Ok(StepResult::Next),
                "b" | "back" => return Ok(StepResult::Back),
                "q" | "quit" | "n" | "no" | "" => return Ok(StepResult::Abort),
                _ => println!("  Please enter y or n."),
            }
        }
    }
}
