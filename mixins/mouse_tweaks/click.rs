// Handle button clicks in Options > Mods > Mouse Tweaks.
fn mouse_tweaks_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    crate::game::mod_mouse_tweaks_glue::on_click(game, mod_id, button);
}
