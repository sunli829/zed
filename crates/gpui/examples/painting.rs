use gpui::{
    Application, Background, Bounds, ColorSpace, Context, MouseDownEvent, Path, PathBuilder,
    PathStyle, Pixels, Point, Render, StrokeOptions, Window, WindowBounds, WindowOptions, canvas,
    div, linear_color_stop, linear_gradient, point, prelude::*, px, rgb, size,
};
use rand::Rng;

const DEFAULT_WINDOW_WIDTH: Pixels = px(800.0);
const DEFAULT_WINDOW_HEIGHT: Pixels = px(600.0);

struct PaintingViewer {
    default_lines: Vec<(Path<Pixels>, Background)>,
    lines: Vec<Vec<Point<Pixels>>>,
    start: Point<Pixels>,
    _painting: bool,
}

impl PaintingViewer {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let mut lines = vec![];

        // draw a ⭐
        let mut rng = rand::thread_rng();
        for i in 0..5000 {
            let offset_x = rng.gen_range(-400.0..400.0);
            let offset_y = rng.gen_range(-300.0..300.0);

            // 随机颜色
            let r = rng.gen_range(0..=255);
            let g = rng.gen_range(0..=255);
            let b = rng.gen_range(0..=255);

            let mut builder = PathBuilder::fill();
            builder.move_to(point(px(350. + offset_x), px(100. + offset_y)));
            builder.line_to(point(px(370. + offset_x), px(160. + offset_y)));
            builder.line_to(point(px(430. + offset_x), px(160. + offset_y)));
            builder.line_to(point(px(380. + offset_x), px(200. + offset_y)));
            builder.line_to(point(px(400. + offset_x), px(260. + offset_y)));
            builder.line_to(point(px(350. + offset_x), px(220. + offset_y)));
            builder.line_to(point(px(300. + offset_x), px(260. + offset_y)));
            builder.line_to(point(px(320. + offset_x), px(200. + offset_y)));
            builder.line_to(point(px(270. + offset_x), px(160. + offset_y)));
            builder.line_to(point(px(330. + offset_x), px(160. + offset_y)));
            builder.line_to(point(px(350. + offset_x), px(100. + offset_y)));
            let path = builder.build().unwrap();

            lines.push((
                path,
                linear_gradient(
                    180.,
                    linear_color_stop(rgb((r << 16) | (g << 8) | b), 0.7),
                    linear_color_stop(rgb(0xD56D0C), 1.),
                )
                .color_space(ColorSpace::Oklab),
            ));
        }

        Self {
            default_lines: lines.clone(),
            lines: vec![],
            start: point(px(0.), px(0.)),
            _painting: false,
        }
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.lines.clear();
        cx.notify();
    }
}

impl Render for PaintingViewer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.request_animation_frame();

        let default_lines = self.default_lines.clone();
        let lines = self.lines.clone();
        let window_size = window.bounds().size;
        let scale = 1.0; //window_size.width / DEFAULT_WINDOW_WIDTH;
        div()
            .font_family(".SystemUIFont")
            .bg(gpui::white())
            .size_full()
            .p_4()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .justify_between()
                    .items_center()
                    .child("Mouse down any point and drag to draw lines (Hold on shift key to draw straight lines)")
                    .child(
                        div()
                            .id("clear")
                            .child("Clean up")
                            .bg(gpui::black())
                            .text_color(gpui::white())
                            .active(|this| this.opacity(0.8))
                            .flex()
                            .px_3()
                            .py_1()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.clear(cx);
                            })),
                    ),
            )
            .child(
                div()
                    .size_full()
                    .child(
                        canvas(
                            move |_, _, _| {},
                            move |_, _, window, _| {
                                for (path, color) in default_lines {
                                    window.paint_path(path.clone().scale(scale), color);
                                }

                                for points in lines {
                                    if points.len() < 2 {
                                        continue;
                                    }

                                    let mut builder = PathBuilder::stroke(px(1.));
                                    for (i, p) in points.into_iter().enumerate() {
                                        if i == 0 {
                                            builder.move_to(p);
                                        } else {
                                            builder.line_to(p);
                                        }
                                    }

                                    if let Ok(path) = builder.build() {
                                        window.paint_path(path, gpui::black());
                                    }
                                }
                            },
                        )
                        .size_full(),
                    )
                    .on_mouse_down(
                        gpui::MouseButton::Left,
                        cx.listener(|this, ev: &MouseDownEvent, _, _| {
                            this._painting = true;
                            this.start = ev.position;
                            let path = vec![ev.position];
                            this.lines.push(path);
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, ev: &gpui::MouseMoveEvent, _, cx| {
                        if !this._painting {
                            return;
                        }

                        let is_shifted = ev.modifiers.shift;
                        let mut pos = ev.position;
                        // When holding shift, draw a straight line
                        if is_shifted {
                            let dx = pos.x - this.start.x;
                            let dy = pos.y - this.start.y;
                            if dx.abs() > dy.abs() {
                                pos.y = this.start.y;
                            } else {
                                pos.x = this.start.x;
                            }
                        }

                        if let Some(path) = this.lines.last_mut() {
                            path.push(pos);
                        }

                        cx.notify();
                    }))
                    .on_mouse_up(
                        gpui::MouseButton::Left,
                        cx.listener(|this, _, _, _| {
                            this._painting = false;
                        }),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions {
                focus: true,
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
                    cx,
                ))),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| PaintingViewer::new(window, cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
