// Pure runtime logic for Mouse Tweaks mod.
// Compiled both as standalone library (for unit tests) and inside Horizon client.
use std::cell::RefCell;

pub const ID: &str = "mouse_tweaks";
pub const NAME: &str = "Mouse Tweaks";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "RMB drag deposit, Shift+LMB drag quick-transfer, and scroll wheel item move.";

/// User settings for Mouse Tweaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    pub enabled: bool,
    pub rmb_drag: bool,
    pub shift_drag: bool,
    pub scroll_wheel: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
            rmb_drag: true,
            shift_drag: true,
            scroll_wheel: true,
        }
    }
}

impl Settings {
    pub fn labels(&self) -> Vec<String> {
        vec![
            format!(
                "Mouse Tweaks: {}",
                if self.enabled { "An" } else { "Aus" }
            ),
            format!(
                "RMB Drag (Verteilen): {}",
                if self.rmb_drag { "An" } else { "Aus" }
            ),
            format!(
                "Shift Drag (Schnell-Bewegen): {}",
                if self.shift_drag { "An" } else { "Aus" }
            ),
            format!(
                "Scrollrad (Item-Transfer): {}",
                if self.scroll_wheel { "An" } else { "Aus" }
            ),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.enabled = !self.enabled,
            1 => self.rmb_drag = !self.rmb_drag,
            2 => self.shift_drag = !self.shift_drag,
            3 => self.scroll_wheel = !self.scroll_wheel,
            _ => {}
        }
    }

    pub fn stored(&self) -> Vec<(&'static str, String)> {
        vec![
            ("enabled", (if self.enabled { "1" } else { "0" }).to_string()),
            ("rmb_drag", (if self.rmb_drag { "1" } else { "0" }).to_string()),
            (
                "shift_drag",
                (if self.shift_drag { "1" } else { "0" }).to_string(),
            ),
            (
                "scroll_wheel",
                (if self.scroll_wheel { "1" } else { "0" }).to_string(),
            ),
        ]
    }

    pub fn from_loader<F: Fn(&str) -> Option<String>>(loader: F) -> Self {
        let mut s = Self::default();
        if let Some(v) = loader("enabled") {
            s.enabled = v == "1" || v.eq_ignore_ascii_case("true");
        }
        if let Some(v) = loader("rmb_drag") {
            s.rmb_drag = v == "1" || v.eq_ignore_ascii_case("true");
        }
        if let Some(v) = loader("shift_drag") {
            s.shift_drag = v == "1" || v.eq_ignore_ascii_case("true");
        }
        if let Some(v) = loader("scroll_wheel") {
            s.scroll_wheel = v == "1" || v.eq_ignore_ascii_case("true");
        }
        s
    }
}

thread_local! {
    static SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with_borrow_mut(|s| *s.get_or_insert_with(|| Settings::from_loader(get)))
}

pub fn set_settings(new_settings: Settings) {
    SETTINGS.with_borrow_mut(|s| *s = Some(new_settings));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let s = Settings::default();
        assert!(s.enabled);
        assert!(s.rmb_drag);
        assert!(s.shift_drag);
        assert!(s.scroll_wheel);
        assert_eq!(s.labels().len(), 4);
    }

    #[test]
    fn test_click_toggles() {
        let mut s = Settings::default();
        s.click(0);
        assert!(!s.enabled);
        s.click(0);
        assert!(s.enabled);

        s.click(1);
        assert!(!s.rmb_drag);
        s.click(2);
        assert!(!s.shift_drag);
        s.click(3);
        assert!(!s.scroll_wheel);
    }

    #[test]
    fn test_storage_roundtrip() {
        let mut s = Settings::default();
        s.enabled = true;
        s.rmb_drag = false;
        s.shift_drag = true;
        s.scroll_wheel = false;

        let stored = s.stored();
        let loaded = Settings::from_loader(|k| {
            stored.iter().find(|(key, _)| *key == k).map(|(_, v)| v.clone())
        });
        assert_eq!(s, loaded);
    }
}
