use orbtk::prelude::*;

widget!(View);

impl Template for View {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("main view").child(
            Stack::new()
                .spacing(8.0)
                .orientation("vertical")
                .h_align("center")
                .child(
                    TextBlock::new()
                        .text("Text Block 1")
                        .font_size(32.0)
                        .build(ctx),
                )
                .child(
                    TextBox::new()
                        .v_align("center")
                        .font_size(32.0)
                        .text(String16::from("Tahmid jSadik"))
                        .name("input box ")
                        .width(400.0)
                        .height(80.0)
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk- Minimal")
                .position((100.0, 100.0))
                .size(1000.0, 800.0)
                .child(View::new().size(600.0, 400.0).build(ctx))
                .build(ctx)
        })
        .run();
}
