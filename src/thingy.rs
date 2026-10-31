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
    /// Create an action from an executable and optional whitespace-separated arguments.
    /// Shell quoting and expansions are not interpreted.
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
/// Manages an in-memory registry of quick-access actions.
///
/// D-Bus exposure is planned but not implemented; [`Thingy::run`] reports that
/// limitation instead of claiming that a service has started.
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

    /// Start an action by ID as a background process without invoking a shell.
    ///
    /// Returns `true` only when the executable was found and the OS accepted
    /// the spawn request. It does not report the child process's later exit.
    pub fn activate(&self, id: &str) -> bool {
        if let Some(action) = self.actions.get(id) {
            self.spawn_command(&action.command)
        } else {
            false
        }
    }

    /// Spawn an executable and literal whitespace-separated arguments.
    fn spawn_command(&self, command: &str) -> bool {
        let mut words = command.split_whitespace();
        let Some(program) = words.next() else {
            return false;
        };
        std::process::Command::new(program)
            .args(words)
            .spawn()
            .is_ok()
    }

    /// Start the Thingy D-Bus service.
    ///
    /// Returns `Unsupported` until D-Bus integration and its event loop exist.
    pub fn run(&self) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Thingy D-Bus service is not implemented",
        ))
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
