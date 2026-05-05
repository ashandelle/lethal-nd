#[macro_export]
macro_rules! join_menu {
    (
        $lang:ident, $joinmenu:lifetime, $state:ident, $mousegrab:ident, $debugtimer:ident, $address:ident, $port:ident, $height:ident, $screen:ident,
        $large_button_skin:ident, $small_button_skin:ident, $input_skin:ident,
        $hash1:expr, $hash2:expr
    ) => {
        clear_background(LIGHTGRAY);

        let spacing: f32 = 10.0;
        let address_width: f32 = 150.0;
        let port_width: f32 = 50.0;

        root_ui().push_skin(&$large_button_skin);

        let back_size = root_ui().calc_size($lang.back);
        if root_ui().button(Vec2::new(spacing, $height - (spacing + back_size.y)), $lang.back) {
            $state = ClientState::MainMenu;
            statechanged(&$state, &mut $mousegrab, &mut $debugtimer);
            break $joinmenu;
        }

        root_ui().pop_skin();
        root_ui().push_skin(&$small_button_skin);

        let join_size = root_ui().calc_size($lang.join);

        root_ui().pop_skin();
        root_ui().push_skin(&$input_skin);

        let address_size = Vec2::new(address_width, join_size.y);
        InputText::new($hash1)
        // .label(lang.address)
        .position(($screen - address_size - Vec2::new(0.0, spacing)) / 2.0)
        .size(address_size)
        .ui(&mut root_ui(), $address);

        let port_size = Vec2::new(port_width, join_size.y);
        InputText::new($hash2)
        // .label(lang.port)
        .position(($screen - address_size + Vec2::new(0.0, spacing)) / 2.0 + Vec2::new(0.0, join_size.y))
        .size(port_size)
        .ui(&mut root_ui(), $port);

        root_ui().pop_skin();
        root_ui().push_skin(&$small_button_skin);

        if root_ui().button(($screen - address_size + Vec2::new(0.0, spacing)) / 2.0 + Vec2::new(address_size.x - join_size.x, join_size.y), $lang.join) {
            // state = ClientState::MainMenu;
            let addr: Ipv4Addr = match $address.parse() {
                Ok(addr) => addr,
                Err(err) => {
                    $state = ClientState::Disconnected {
                        reason: err.to_string()
                    };
                    statechanged(&$state, &mut $mousegrab, &mut $debugtimer);
                    break $joinmenu;
                },
            };
            let pt: u16 = match $port.parse() {
                Ok(port) => port,
                Err(err) => {
                    $state = ClientState::Disconnected {
                        reason: err.to_string()
                    };
                    statechanged(&$state, &mut $mousegrab, &mut $debugtimer);
                    break $joinmenu;
                },
            };

            let socket: SocketAddr = SocketAddr::new(IpAddr::V4(addr), pt);

            $state = ClientState::Connecting { address: socket };
            statechanged(&$state, &mut $mousegrab, &mut $debugtimer);
        }

        root_ui().pop_skin();
    }
}