mod search_input;

use gpui::{
    App, Application, Bounds, Context, Entity, Focusable, KeyBinding, Render, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};

use search_input::{
    Backspace, Copy, Cut, Delete, End, Home, Left, Paste, Right, SearchInput, SelectAll, SelectLeft,
    SelectRight, ShowCharacterPalette,
};

struct Root {
    search: Entity<SearchInput>,
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .size_full()
            .p_4()
            .items_center()
            .bg(rgb(0x1e1e1e))
            .text_lg()
            .text_color(rgb(0xe0e0e0))
            .child(
                div()
                    .w_full()
                    .max_w(px(400.0))
                    .child(self.search.clone()),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("ctrl-a", SelectAll, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("ctrl-v", Paste, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("ctrl-c", Copy, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("ctrl-x", Cut, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
        ]);

        let window = cx
            .open_window(
                WindowOptions {
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("gpui-browser by amaan".into()),
                        appears_transparent: false,
                        traffic_light_position: None,
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(520.0), px(400.0)),
                        cx,
                    ))),
                    ..Default::default()
                },
                |_, cx| {
                    let search = cx.new(|cx| {
                        SearchInput::new(cx, "Search or enter address...")
                    });
                    cx.new(|_| Root { search })
                },
            )
            .unwrap();

        window
            .update(cx, |root, window, cx| {
                window.focus(&root.search.focus_handle(cx));
                cx.activate(true);
            })
            .unwrap();
    });
}
