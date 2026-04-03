//! Wizard step 5 — Bundle selection.
//!
//! Reads the bundle list from the cloned store catalog when available.
//! Falls back to the built-in defaults if the catalog is missing or fails to parse.

use crate::capability::BootstrapMode;
use crate::catalog_reader;
use crate::error::FsInitError;
use crate::keys;
use crate::wizard::{default_bundles, prompt, BundleChoice, StepResult, WizardState, WizardStep};

pub struct BundleStep;

impl WizardStep for BundleStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_BUNDLE_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        let available = load_available(state);

        println!();
        println!("{}", keys::INIT_STEP_BUNDLE_PROMPT);
        println!();
        for (i, bundle) in available.iter().enumerate() {
            println!("  [{}] {}", i + 1, bundle.name);
            println!("      {}", bundle.description);
        }
        println!();

        let default_idx = default_bundle_idx(state.capability.mode, &available);
        if let Some(d) = default_idx {
            println!("  Default: [{}] {}", d + 1, available[d].name);
        }
        println!("  [b] Back   [q] Quit");
        println!();

        loop {
            let input = prompt(keys::INIT_PROMPT_CHOICE)?;
            match input.as_str() {
                "b" | "back" => return Ok(StepResult::Back),
                "q" | "quit" => return Ok(StepResult::Abort),
                "" => {
                    if let Some(d) = default_idx {
                        state.selected_bundle = Some(available[d].clone());
                        return Ok(StepResult::Next);
                    }
                    println!("{}", keys::INIT_INVALID_CHOICE);
                }
                s => match parse_choice(s, available.len()) {
                    Some(idx) => {
                        state.selected_bundle = Some(available[idx].clone());
                        return Ok(StepResult::Next);
                    }
                    None => println!("{}", keys::INIT_INVALID_CHOICE),
                },
            }
        }
    }
}

// ── Bundle loading ────────────────────────────────────────────────────────────

fn load_available(state: &WizardState) -> Vec<BundleChoice> {
    // Try reading from the cloned store catalog first.
    if state.store_dir.exists() {
        println!("{}", keys::INIT_STEP_BUNDLE_LOADING);
        let catalog_bundles = catalog_reader::load_bundles(&state.store_dir);
        if !catalog_bundles.is_empty() {
            let choices: Vec<BundleChoice> = catalog_bundles
                .into_iter()
                .map(|b| BundleChoice {
                    id: b.id,
                    name: b.name,
                    description: b.description,
                    requires_display: b.requires_display,
                })
                .collect();

            let filtered = filter_by_mode(state.capability.mode, choices);
            if !filtered.is_empty() {
                return filtered;
            }
        }
    }

    // Fall back to built-in defaults.
    println!("{}", keys::INIT_STEP_BUNDLE_USING_DEFAULTS);
    filter_by_mode(state.capability.mode, default_bundles())
}

fn filter_by_mode(mode: BootstrapMode, bundles: Vec<BundleChoice>) -> Vec<BundleChoice> {
    bundles
        .into_iter()
        .filter(|b| mode == BootstrapMode::Gui || !b.requires_display)
        .collect()
}

fn default_bundle_idx(mode: BootstrapMode, available: &[BundleChoice]) -> Option<usize> {
    let preferred = match mode {
        BootstrapMode::Gui => "freeSynergy.bundle.workstation",
        BootstrapMode::Tui | BootstrapMode::Headless => "freeSynergy.bundle.server",
    };
    available.iter().position(|b| b.id == preferred)
}

fn parse_choice(s: &str, len: usize) -> Option<usize> {
    s.parse::<usize>().ok().and_then(|n| {
        if n >= 1 && n <= len {
            Some(n - 1)
        } else {
            None
        }
    })
}
