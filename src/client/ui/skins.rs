#[macro_export]
macro_rules! skins {
    (
        $title_style:ident, $large_button_style:ident, $small_button_style:ident, $input_style:ident,
        $title_skin:ident, $large_button_skin:ident, $small_button_skin:ident, $input_skin:ident
    ) => {
        let $title_skin = Skin {
            label_style: $title_style.clone(),
            ..root_ui().default_skin()
        };

        let $large_button_skin = Skin {
            button_style: $large_button_style.clone(),
            label_style: $large_button_style.clone(),
            ..root_ui().default_skin()
        };

        let $small_button_skin = Skin {
            button_style: $small_button_style.clone(),
            label_style: $small_button_style.clone(),
            ..root_ui().default_skin()
        };

        let $input_skin = Skin {
            label_style: $input_style.clone(),
            ..root_ui().default_skin()
        };
    }
}