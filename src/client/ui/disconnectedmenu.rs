#[macro_export]
macro_rules! disconnected_menu {
    (
        $lang:ident, $state:ident, $mousegrab:ident, $debugtimer:ident, $reason:ident, $height:ident, $screen:ident,
        $large_button_skin:ident
    ) => {
        clear_background(LIGHTGRAY);

        let spacing: f32 = 10.0;

        root_ui().push_skin(&$large_button_skin);

        let back_size = root_ui().calc_size($lang.back);
        if root_ui().button(Vec2::new(spacing, $height - (spacing + back_size.y)), $lang.back) {
            $state = ClientState::MainMenu;
            statechanged(&$state, &mut $mousegrab, &mut $debugtimer);
        }

        root_ui().pop_skin();
    }
}