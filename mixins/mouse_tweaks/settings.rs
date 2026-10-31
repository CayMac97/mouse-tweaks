// Mouse Tweaks settings page buttons in Options > Mods.
fn mouse_tweaks_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    crate::game::mod_mouse_tweaks_glue::on_settings(game, mod_id, buttons);
}
