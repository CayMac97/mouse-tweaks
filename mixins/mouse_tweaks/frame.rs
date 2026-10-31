// Hook helper: native_frame
fn mouse_tweaks_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::mod_mouse_tweaks_glue::on_frame(game, elapsed);
}
