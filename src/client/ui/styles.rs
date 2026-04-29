#[macro_export]
macro_rules! styles {
    ($title_style:ident, $large_button_style:ident, $small_button_style:ident, $input_style:ident) => {
        let $title_style = root_ui().style_builder()
            // .font(&font).unwrap()
            .text_color(WHITE)
            .font_size(60)
            .build();

        let $large_button_style = root_ui().style_builder()
            // .background(button_background)
            // .background_clicked(button_clicked_background)
            // .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(16.0, 16.0, 8.0, 8.0))
            // .font(&font).unwrap()
            .color(DARKGRAY)
            .color_hovered(GRAY)
            .text_color(WHITE)
            .font_size(30)
            .build();

        let $small_button_style = root_ui().style_builder()
            // .background(button_background)
            // .background_clicked(button_clicked_background)
            // .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
            // .font(&font).unwrap()
            .color(DARKGRAY)
            .color_hovered(GRAY)
            .text_color(WHITE)
            .font_size(20)
            .build();

        let $input_style = root_ui().style_builder()
            // .background(button_background)
            // .background_clicked(button_clicked_background)
            // .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
            // .font(&font).unwrap()
            .color(GRAY)
            .text_color(WHITE)
            .font_size(20)
            .build();
    }
}