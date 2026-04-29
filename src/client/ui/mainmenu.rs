#[macro_export]
macro_rules! main_menu {
    (
        $lang:ident, $state:ident, $screen:ident,
        $title_skin:ident, $button_skin:ident
    ) => {
        clear_background(LIGHTGRAY);

        let spacing: f32 = 10.0;

        root_ui().push_skin(&$title_skin);

        let title_size = root_ui().calc_size($lang.title);

        root_ui().pop_skin();
        root_ui().push_skin(&$button_skin);

        let play_size = root_ui().calc_size($lang.play);
        let settings_size = root_ui().calc_size($lang.settings);
        let exit_size = root_ui().calc_size($lang.exit);

        let vert = title_size.y + spacing + play_size.y + spacing + settings_size.y + spacing + exit_size.y;
        let mut curr = vert / 2.0;

        curr -= exit_size.y / 2.0;
        if root_ui().button(($screen - exit_size) / 2.0 + Vec2::new(0.0,curr), $lang.exit) {
            $state = ClientState::Exit;
            printstate(&$state);
        }
        curr -= spacing + (exit_size.y + settings_size.y) / 2.0;
        if root_ui().button(($screen - settings_size) / 2.0 + Vec2::new(0.0,curr), $lang.settings) {
            $state = ClientState::MainSettings;
            printstate(&$state);
        }
        curr -= spacing + (settings_size.y + play_size.y) / 2.0;
        if root_ui().button(($screen - play_size) / 2.0 + Vec2::new(0.0,curr), $lang.play) {
            $state = ClientState::JoinMenu{
                address: Default::default(),
                port: "5000".to_string(),
            };
            printstate(&$state);
        }

        root_ui().pop_skin();
        root_ui().push_skin(&$title_skin);

        curr -= spacing + (play_size.y + title_size.y) / 2.0;
        root_ui().label(($screen - title_size) / 2.0 + Vec2::new(0.0,curr), $lang.title);

        root_ui().pop_skin();
    }
}