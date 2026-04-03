//! FTL key constants with hardcoded English fallback values.
//!
//! Every user-facing string must be declared here.
//! The FTL key name is documented in the comment above each constant.
//! The value is the English fallback embedded in the binary.
//! When fs-i18n is available at runtime, the FTL key is used instead.

// ── General ───────────────────────────────────────────────────────────────────

/// FTL key: `init-title`
pub const INIT_TITLE: &str = "FreeSynergy Init";

/// FTL key: `init-version`
pub const INIT_DIVIDER: &str = "────────────────────────────────────────────────";

/// FTL key: `init-abort-hint`
pub const INIT_ABORT_HINT: &str = "Press Ctrl+C at any time to abort.";

/// FTL key: `init-prompt-continue`
pub const INIT_PROMPT_CONTINUE: &str = "Press Enter to continue…";

/// FTL key: `init-prompt-choice`
pub const INIT_PROMPT_CHOICE: &str = "Enter number: ";

/// FTL key: `init-invalid-choice`
pub const INIT_INVALID_CHOICE: &str = "Invalid choice. Please try again.";

// ── Capability detection ──────────────────────────────────────────────────────

/// FTL key: `init-detecting-capabilities`
pub const INIT_DETECTING_CAPABILITIES: &str = "Detecting system capabilities…";

/// FTL key: `init-capability-os`
pub const INIT_CAPABILITY_OS: &str = "  OS:             ";

/// FTL key: `init-capability-arch`
pub const INIT_CAPABILITY_ARCH: &str = "  Architecture:   ";

// ── Wizard steps ──────────────────────────────────────────────────────────────

/// FTL key: `init-step-welcome-title`
pub const INIT_STEP_WELCOME_TITLE: &str = "Welcome";

/// FTL key: `init-step-welcome-body`
pub const INIT_STEP_WELCOME_BODY: &str =
    "This wizard will guide you through installing FreeSynergy on this node.\n\
     It will clone the official store and help you choose what to install.";

/// FTL key: `init-step-capability-title`
pub const INIT_STEP_CAPABILITY_TITLE: &str = "System Capabilities";

/// FTL key: `init-step-store-load-title`
pub const INIT_STEP_STORE_LOAD_TITLE: &str = "Store Catalog";

/// FTL key: `init-step-store-load-cloning`
pub const INIT_STEP_STORE_LOAD_CLONING: &str = "  Cloning store catalog…";

/// FTL key: `init-step-store-load-exists`
pub const INIT_STEP_STORE_LOAD_EXISTS: &str = "  Store already present — skipping clone.";

/// FTL key: `init-step-store-load-done`
pub const INIT_STEP_STORE_LOAD_DONE: &str = "  Store ready.";

/// FTL key: `init-step-store-load-failed`
pub const INIT_STEP_STORE_LOAD_FAILED: &str =
    "  Warning: could not clone store. Using built-in bundle defaults.";

/// FTL key: `init-step-engine-title`
pub const INIT_STEP_ENGINE_TITLE: &str = "Render Engine";

/// FTL key: `init-step-engine-prompt`
pub const INIT_STEP_ENGINE_PROMPT: &str = "Choose the render engine for the desktop UI.\n\
     (Only relevant if you install a bundle with a desktop.)";

/// FTL key: `init-step-bundle-title`
pub const INIT_STEP_BUNDLE_TITLE: &str = "Bundle Selection";

/// FTL key: `init-step-bundle-prompt`
pub const INIT_STEP_BUNDLE_PROMPT: &str = "Choose a bundle to install:";

/// FTL key: `init-step-bundle-loading`
pub const INIT_STEP_BUNDLE_LOADING: &str = "  Loading bundles from store catalog…";

/// FTL key: `init-step-bundle-using-defaults`
pub const INIT_STEP_BUNDLE_USING_DEFAULTS: &str = "  (using built-in bundle defaults)";

/// FTL key: `init-step-confirm-title`
pub const INIT_STEP_CONFIRM_TITLE: &str = "Confirm Installation";

/// FTL key: `init-step-confirm-bundle`
pub const INIT_STEP_CONFIRM_BUNDLE: &str = "  Bundle:          ";

/// FTL key: `init-step-confirm-engine`
pub const INIT_STEP_CONFIRM_ENGINE: &str = "  Render engine:   ";

/// FTL key: `init-step-confirm-target`
pub const INIT_STEP_CONFIRM_TARGET: &str = "  Install target:  ";

/// FTL key: `init-step-confirm-question`
pub const INIT_STEP_CONFIRM_QUESTION: &str = "Proceed? [y/N]: ";

/// FTL key: `init-step-progress-title`
pub const INIT_STEP_PROGRESS_TITLE: &str = "Installing";

/// FTL key: `init-step-progress-step-started`
pub const INIT_STEP_PROGRESS_STEP_STARTED: &str = "    [";

/// FTL key: `init-step-progress-step-ok`
pub const INIT_STEP_PROGRESS_STEP_OK: &str = " done";

/// FTL key: `init-step-progress-step-skipped`
pub const INIT_STEP_PROGRESS_STEP_SKIPPED: &str = " skipped";

/// FTL key: `init-step-progress-step-failed`
pub const INIT_STEP_PROGRESS_STEP_FAILED: &str = " FAILED";

/// FTL key: `init-step-progress-installing`
pub const INIT_STEP_PROGRESS_INSTALLING: &str = "  Installing: ";

/// FTL key: `init-step-progress-install-ok`
pub const INIT_STEP_PROGRESS_INSTALL_OK: &str = "  All packages installed successfully.";

/// FTL key: `init-step-progress-install-failed`
pub const INIT_STEP_PROGRESS_INSTALL_FAILED: &str = "  Installation failed: ";

/// FTL key: `init-step-progress-no-bundle`
pub const INIT_STEP_PROGRESS_NO_BUNDLE: &str =
    "  No bundle selected — skipping package installation.";

/// FTL key: `init-step-progress-adapter-note`
pub const INIT_STEP_PROGRESS_ADAPTER_NOTE: &str =
    "  (adapter packages are installed automatically)";

// ── Legacy keys kept for clone-only mode ─────────────────────────────────────

/// FTL key: `init-step-progress-cloning-store`
pub const INIT_STEP_PROGRESS_CLONING_STORE: &str = "  Cloning store catalog…";

/// FTL key: `init-step-progress-clone-ok`
pub const INIT_STEP_PROGRESS_CLONE_OK: &str = "  Store ready.";

/// FTL key: `init-step-progress-clone-exists`
pub const INIT_STEP_PROGRESS_CLONE_EXISTS: &str = "  Store already present — skipping clone.";

/// FTL key: `init-step-done-title`
pub const INIT_STEP_DONE_TITLE: &str = "Done";

/// FTL key: `init-step-done-body`
pub const INIT_STEP_DONE_BODY: &str = "FreeSynergy has been bootstrapped successfully.";

/// FTL key: `init-step-done-store-path`
pub const INIT_STEP_DONE_STORE_PATH: &str = "  Store path:      ";

/// FTL key: `init-step-done-next-steps`
pub const INIT_STEP_DONE_NEXT_STEPS: &str = "Next steps:";

/// FTL key: `init-step-done-manager-hint`
pub const INIT_STEP_DONE_MANAGER_HINT: &str =
    "  1. Run 'fs-manager start' to configure your installed services.";

/// FTL key: `init-step-done-store-hint`
pub const INIT_STEP_DONE_STORE_HINT: &str =
    "  2. Open the Store to browse and install additional packages.";

// ── Target ────────────────────────────────────────────────────────────────────

/// FTL key: `init-target-container`
pub const INIT_TARGET_CONTAINER: &str = "Container (Podman / Docker)";

/// FTL key: `init-target-rpm`
pub const INIT_TARGET_RPM: &str = "RPM package";

/// FTL key: `init-target-deb`
pub const INIT_TARGET_DEB: &str = "DEB package";

/// FTL key: `init-target-appimage`
pub const INIT_TARGET_APPIMAGE: &str = "AppImage";
