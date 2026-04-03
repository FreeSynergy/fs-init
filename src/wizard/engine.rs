//! Wizard step 4 — Render engine selection.

use crate::capability::BootstrapMode;
use crate::error::FsInitError;
use crate::keys;
use crate::wizard::{default_engines, prompt, EngineChoice, StepResult, WizardState, WizardStep};

pub struct EngineStep;

impl WizardStep for EngineStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_ENGINE_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        let available = available_engines(state.capability.mode);

        println!();
        println!("{}", keys::INIT_STEP_ENGINE_PROMPT);
        println!();
        for (i, engine) in available.iter().enumerate() {
            println!("  [{}] {} — {}", i + 1, engine.name, engine.description);
        }
        println!();

        let default_idx = default_engine_idx(state.capability.mode, &available);
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
                        state.selected_engine = Some(available[d].clone());
                        return Ok(StepResult::Next);
                    }
                    println!("{}", keys::INIT_INVALID_CHOICE);
                }
                s => match parse_choice(s, available.len()) {
                    Some(idx) => {
                        state.selected_engine = Some(available[idx].clone());
                        return Ok(StepResult::Next);
                    }
                    None => println!("{}", keys::INIT_INVALID_CHOICE),
                },
            }
        }
    }
}

fn available_engines(mode: BootstrapMode) -> Vec<EngineChoice> {
    default_engines()
        .into_iter()
        .filter(|e| mode == BootstrapMode::Gui || !e.requires_display)
        .collect()
}

fn default_engine_idx(mode: BootstrapMode, available: &[EngineChoice]) -> Option<usize> {
    let preferred = match mode {
        BootstrapMode::Gui => "iced",
        BootstrapMode::Tui => "tui",
        BootstrapMode::Headless => "none",
    };
    available.iter().position(|e| e.id == preferred)
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
