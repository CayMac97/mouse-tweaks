// Register Mouse Tweaks in Options > Mods.
fn mouse_tweaks_list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::mod_mouse_tweaks_glue::on_list(mods);
}
