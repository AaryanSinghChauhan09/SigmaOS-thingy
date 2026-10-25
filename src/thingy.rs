//! Thingy core module.
//!
//! Single responsibility: manage system quick-access actions and applet lifecycle.
//! Inspired by linuxmint/thingy.

use std::collections::HashMap;

/// A quick-action registered in the Thingy panel.
#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub command: String,
}

impl Action {
    /// Create a new action.
    pub fn new(id: impl Into<String>, label: impl Into<String>, command: impl Into<String>) -> Self {
        Action {
            id: id.into(),
            label: label.into(),
            icon: None,
            command: command.into(),
        }
    }

    /// Attach an icon name to this action.
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// The Thingy utility panel.
///
/// Manages a registry of quick-access actions and exposes them to the
/// SigmaOS shell via D-Bus. Lightweight by design — no persistent state.
pub struct Thingy {
    actions: HashMap<String, Action>,
}

impl Thingy {
    /// Create a new Thingy panel with default system actions.
    pub fn new() -> Self {
        let mut t = Thingy {
            actions: HashMap::new(),
        };
        t.register_defaults();
        t
    }

    /// Register the built-in default actions.
    fn register_defaults(&mut self) {
        self.register(
            Action::new("lock-screen", "Lock Screen", "loginctl lock-session")
                .with_icon("system-lock-screen"),
        );
        self.register(
            Action::new("settings", "System Settings", "sigma-settings")
                .with_icon("preferences-system"),
        );
        self.register(
            Action::new("terminal", "Terminal", "sigma-terminal")
                .with_icon("utilities-terminal"),
        );
        self.register(
            Action::new("files", "Files", "sigma-files").with_icon("system-file-manager"),
        );
    }

    /// Add or replace an action in the registry.
    pub fn register(&mut self, action: Action) {
        self.actions.insert(action.id.clone(), action);
    }

    /// Remove an action by its ID. Returns the removed action if it existed.
    pub fn unregister(&mut self, id: &str) -> Option<Action> {
        self.actions.remove(id)
    }

    /// Look up an action by ID.
    pub fn get(&self, id: &str) -> Option<&Action> {
        self.actions.get(id)
    }

    /// Execute an action by ID, spawning its command in the background.
    ///
    /// Returns `true` if the action was found and launched, `false` otherwise.
    pub fn activate(&self, id: &str) -> bool {
        if let Some(action) = self.actions.get(id) {
            self.spawn_command(&action.command);
            true
        } else {
            false
        }
    }

    /// Spawn the given command string as a background process.
    fn spawn_command(&self, cmd: &str) {
        // TODO: replace with a proper process spawner
        let _ = std::process::Command::new("sh")
            .args(["-c", cmd])
            .spawn();
    }

    /// Start the Thingy D-Bus service and enter the event loop.
    pub fn run(&self) {
        // TODO: connect to D-Bus, export object, run event loop
    }

    /// Return an iterator over all registered actions.
    pub fn actions(&self) -> impl Iterator<Item = &Action> {
        self.actions.values()
    }
}

impl Default for Thingy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_actions_registered() {
        let thingy = Thingy::new();
        assert!(thingy.get("lock-screen").is_some());
        assert!(thingy.get("settings").is_some());
        assert!(thingy.get("terminal").is_some());
        assert!(thingy.get("files").is_some());
    }

    #[test]
    fn custom_action_register_and_remove() {
        let mut thingy = Thingy::new();
        let action = Action::new("test-action", "Test", "echo hello");
        thingy.register(action);
        assert!(thingy.get("test-action").is_some());
        let removed = thingy.unregister("test-action");
        assert!(removed.is_some());
        assert!(thingy.get("test-action").is_none());
    }

    #[test]
    fn activate_missing_action_returns_false() {
        let thingy = Thingy::new();
        assert!(!thingy.activate("nonexistent"));
    }
}
