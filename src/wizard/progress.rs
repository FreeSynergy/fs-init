//! Wizard step 7 — Installation progress.
//!
//! Runs the fs-store install pipeline for every required component of the
//! selected bundle.  Adapters are installed automatically by `AdapterInstallStep`
//! inside the pipeline — no extra wiring needed.
//!
//! If no bundle was selected the step is a no-op.

use fs_store::{InstallContext, InstallKind, InstallRequest, InstallTarget, Pipeline, StoreSource};

use crate::catalog_reader::{self, CatalogComponent};
use crate::error::FsInitError;
use crate::keys;
use crate::wizard::{StepResult, WizardState, WizardStep};

pub struct ProgressStep;

impl WizardStep for ProgressStep {
    fn title(&self) -> &'static str {
        keys::INIT_STEP_PROGRESS_TITLE
    }

    fn run(&self, state: &mut WizardState) -> Result<StepResult, FsInitError> {
        println!();

        let Some(bundle) = &state.selected_bundle else {
            println!("{}", keys::INIT_STEP_PROGRESS_NO_BUNDLE);
            println!();
            return Ok(StepResult::Next);
        };

        let bundle_id = bundle.id.clone();
        let store_dir = state.store_dir.clone();
        let install_target = map_target(state.install_target);
        let fs_dir = default_fs_dir();

        // Resolve bundle components from the local catalog.
        let components = if store_dir.exists() {
            catalog_reader::load_bundles(&store_dir)
                .into_iter()
                .find(|b| b.id == bundle_id)
                .map(|b| b.components)
                .unwrap_or_default()
        } else {
            vec![]
        };

        if components.is_empty() {
            println!(
                "  Bundle '{bundle_id}' has no components in catalog — skipping package install."
            );
            println!("  (install packages manually via 'fs-store install <id>')");
            println!();
            return Ok(StepResult::Next);
        }

        println!("{}", keys::INIT_STEP_PROGRESS_ADAPTER_NOTE);
        println!();

        // Install each required component via the pipeline.
        // block_in_place lets us drive async code from this synchronous step
        // while inside the tokio multi-thread runtime.
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                install_components(&components, &install_target, &fs_dir, &store_dir).await
            })
        });

        match result {
            Ok(()) => {
                println!();
                println!("{}", keys::INIT_STEP_PROGRESS_INSTALL_OK);
            }
            Err(e) => {
                println!();
                println!("{}{e}", keys::INIT_STEP_PROGRESS_INSTALL_FAILED);
                return Err(FsInitError::Install(e));
            }
        }

        println!();
        Ok(StepResult::Next)
    }
}

// ── Install helpers ───────────────────────────────────────────────────────────

async fn install_components(
    components: &[CatalogComponent],
    target: &InstallTarget,
    fs_dir: &std::path::Path,
    store_dir: &std::path::Path,
) -> Result<(), String> {
    for comp in components {
        println!("{}{}…", keys::INIT_STEP_PROGRESS_INSTALLING, comp.id);

        let kind = map_kind(&comp.package_type);
        let store_path = resolve_store_path(&comp.id, &comp.package_type, store_dir);

        let request = InstallRequest {
            id: comp.id.clone(),
            name: comp.id.clone(),
            kind,
            version: "latest".to_string(),
            store_path,
            capabilities: vec![],
            icon: None,
        };

        let mut ctx = InstallContext::new(request.clone(), target.clone(), fs_dir.to_path_buf());

        // Subscribe to progress events and print them inline.
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        ctx.progress = Some(tx);

        let pipeline = Pipeline::for_request(&request, target);
        let print_task = tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                use fs_store::PipelineEvent;
                match event {
                    PipelineEvent::StepStarted { step } => {
                        print!("{}{}] ", keys::INIT_STEP_PROGRESS_STEP_STARTED, step);
                    }
                    PipelineEvent::StepCompleted { .. } => {
                        println!("{}", keys::INIT_STEP_PROGRESS_STEP_OK);
                    }
                    PipelineEvent::StepSkipped { reason, .. } => {
                        println!("{}  ({})", keys::INIT_STEP_PROGRESS_STEP_SKIPPED, reason);
                    }
                    PipelineEvent::Failed { step, reason } => {
                        println!("{}", keys::INIT_STEP_PROGRESS_STEP_FAILED);
                        eprintln!("  Error in [{step}]: {reason}");
                    }
                    PipelineEvent::Done => {}
                }
            }
        });

        let result = pipeline.run(&mut ctx).await;
        let _ = print_task.await;

        if let Err(e) = result {
            return Err(format!("{}: {e}", comp.id));
        }
    }
    Ok(())
}

/// Map catalog `[package].type` to `fs-store` `InstallKind`.
fn map_kind(package_type: &str) -> InstallKind {
    match package_type {
        "container" => InstallKind::Container,
        "app" => InstallKind::App,
        "language" => InstallKind::Language,
        "theme" => InstallKind::Theme,
        "bundle" => InstallKind::Bundle,
        _ => InstallKind::Other,
    }
}

/// Map wizard `InstallTarget` to `fs-store` `InstallTarget`.
fn map_target(t: crate::wizard::InstallTarget) -> InstallTarget {
    match t {
        crate::wizard::InstallTarget::Container => InstallTarget::Container,
        crate::wizard::InstallTarget::Rpm => InstallTarget::Rpm,
        crate::wizard::InstallTarget::Deb => InstallTarget::Deb,
        crate::wizard::InstallTarget::AppImage => InstallTarget::AppImage,
    }
}

/// Build the store-relative path for a component.
fn resolve_store_path(id: &str, package_type: &str, store_dir: &std::path::Path) -> Option<String> {
    let _source = StoreSource::Local(store_dir.to_path_buf());
    let candidates = match package_type {
        "container" => vec![format!("packages/containers/{id}")],
        "app" => vec![
            format!("packages/apps/{id}"),
            format!("packages/apps/managers/{id}"),
        ],
        "adapter" => vec![format!("packages/adapters/{id}")],
        _ => vec![],
    };
    candidates.into_iter().next()
}

/// Base directory for FreeSynergy data (`~/.local/share/freesynergy`).
fn default_fs_dir() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    std::path::PathBuf::from(home).join(".local/share/freesynergy")
}
