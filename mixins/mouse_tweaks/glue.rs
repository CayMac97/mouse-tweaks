// Glue between Horizon's `Game` and `mod_mouse_tweaks` runtime state.
use crate::game::Game;
use crate::game::mod_mouse_tweaks as m;
use crate::game::native_hooks::NativeMod;
use std::time::Duration;

pub(super) fn on_frame(game: &mut Game<'_>, _elapsed: Duration) {
    if !game.native_on(m::ID) {
        game.settings.extras.mouse_tweaks = false;
        game.settings.extras.mouse_tweaks_wheel = false;
        return;
    }
    let s = {
        let g = &*game;
        m::settings(|key| g.native_setting(m::ID, key))
    };
    game.settings.extras.mouse_tweaks = s.enabled;
    game.settings.extras.mouse_tweaks_wheel = s.enabled && s.scroll_wheel;
}

pub(super) fn on_list(mods: &mut Vec<NativeMod>) {
    mods.push(NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}

pub(super) fn on_settings(game: &mut Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != m::ID {
        return;
    }
    let g = &*game;
    let settings = m::settings(|key| g.native_setting(m::ID, key));
    buttons.extend(settings.labels());
}

pub(super) fn on_click(game: &mut Game<'_>, mod_id: &str, button: usize) {
    if mod_id != m::ID {
        return;
    }
    let mut settings = {
        let g = &*game;
        m::settings(|key| g.native_setting(m::ID, key))
    };
    settings.click(button);
    m::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}
