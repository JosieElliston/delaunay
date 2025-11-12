mod point;

use eframe::egui::{self, Color32, Key, Pos2, Rect, RichText, Vec2};

fn main() -> eframe::Result {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "delaunay",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

pub(crate) fn lerp(lo: f32, hi: f32, t: f32) -> f32 {
    assert!(lo < hi);
    // assert!((0.0..=1.0).contains(&t));
    // lo * (1.0 - t) + hi * t
    lo + (hi - lo) * t
}

pub(crate) fn inv_lerp(lo: f32, hi: f32, x: f32) -> f32 {
    assert!(lo < hi);
    // assert!((lo..=hi).contains(&x));
    (x - lo) / (hi - lo)
}

#[derive(Debug, Clone, Copy)]
struct Camera {
    x_mid: f32,
    y_mid: f32,
    x_rad: f32,
}
impl Camera {
    fn new(x_mid: f32, y_mid: f32, x_rad: f32) -> Self {
        assert!(x_rad > 0.0);
        Self {
            x_mid,
            y_mid,
            x_rad,
        }
    }

    // fn real_lo(self) -> f32 {
    //     self.x_mid - self.x_rad
    // }

    // fn real_hi(self) -> f32 {
    //     self.x_mid + self.x_rad
    // }

    // fn imag_lo(self) -> f32 {}

    // fn imag_hi(self) -> f32 {}
}
// impl std::ops::AddAssign<(f32, f32)> for Camera {
//     fn add_assign(&mut self, rhs: (f32, f32)) {
//         self.x_mid += rhs.0;
//         self.y_mid += rhs.1;
//     }
// }

struct CameraMap {
    rect: Rect,
    camera: Camera,
}
impl CameraMap {
    fn new(rect: Rect, camera: Camera) -> Self {
        assert!(rect.min.x < rect.max.x);
        assert!(rect.min.y < rect.max.y);
        Self { rect, camera }
    }

    // fn imag_rad(&self) -> f32 {
    //     self.camera.x_rad * self.rect.height() / self.rect.width()
    // }

    // fn imag_lo(&self) -> f32 {
    //     self.camera.y_mid - self.imag_rad()
    // }

    // fn imag_hi(&self) -> f32 {
    //     self.camera.y_mid + self.imag_rad()
    // }

    fn egui_to_cam_x(&self, x: f32) -> Coord {
        // -2.0 * x / self.rect.size().x * self.camera.real_rad
        lerp(
            self.camera.real_lo(),
            self.camera.real_hi(),
            inv_lerp(self.rect.min.x, self.rect.max.x, x),
        )
    }

    fn egui_to_cam_y(&self, y: f32) -> Coord {
        // 2.0 * y * (self.camera.real_rad / self.rect.size().x)
        lerp(
            self.imag_lo(),
            self.imag_hi(),
            1.0 - inv_lerp(self.rect.min.y, self.rect.max.y, y),
        )
    }

    fn egui_to_cam(&self, pos: Pos2) -> Point {
        (self.egui_to_cam_x(pos.x), self.egui_to_cam_y(pos.y))
    }

    fn real_to_x(&self, real: f32) -> f32 {
        lerp(
            self.rect.min.x,
            self.rect.max.x,
            inv_lerp(self.camera.real_lo(), self.camera.real_hi(), real),
        )
    }

    fn imag_to_y(&self, imag: f32) -> f32 {
        lerp(
            self.rect.min.y,
            self.rect.max.y,
            1.0 - inv_lerp(self.imag_lo(), self.imag_hi(), imag),
        )
    }

    // fn complex_to_pos(&self, real: f32, imag: f32) -> Pos2 {
    //     Pos2::new(self.real_to_x(real), self.imag_to_y(imag))
    // }

    fn complex_to_pos(&self, c: (f32, f32)) -> Pos2 {
        Pos2::new(self.real_to_x(c.0), self.imag_to_y(c.1))
    }

    fn window_to_rect(&self, window: impl Into<Window>) -> Rect {
        let window = window.into();
        Rect {
            min: self.complex_to_pos((window.real_lo, window.imag_hi)),
            max: self.complex_to_pos((window.real_hi, window.imag_lo)),
        }
    }

    fn rect_to_window(&self, rect: Rect) -> Window {
        Window::new(
            self.egui_to_cam_x(rect.min.x),
            self.egui_to_cam_x(rect.max.x),
            self.egui_to_cam_y(rect.max.y),
            self.egui_to_cam_y(rect.min.y),
        )
    }
}

struct App {
    tree: Tree,
    stride: usize,
    camera: Camera,
    velocity: Vec2,
    dts: egui::util::History<f32>,
}
impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            tree: Tree::new_leaf(Square::try_new(-4.0, 4.0, -4.0, 4.0).unwrap()),
            stride: 1,
            camera: Camera::new(0.0, 0.0, 2.0),
            velocity: Vec2::ZERO,
            dts: egui::util::History::new(1..100, 0.1),
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show(ctx, |ui| {
                self.dts.add(
                    ctx.input(|input_state| input_state.time),
                    ctx.input(|input_state| input_state.stable_dt),
                );

                // panning stuff
                {
                    let rect = ui.available_rect_before_wrap();
                    let r = ui.allocate_rect(rect, egui::Sense::click_and_drag());

                    let pan_offset = |pan_vec: Vec2, real_rad: f32| -> (f32, f32) {
                        (
                            -2.0 * pan_vec.x / rect.size().x * real_rad,
                            2.0 * pan_vec.y * (real_rad / rect.size().x),
                        )
                    };

                    let dt = ctx.input(|input_state| input_state.stable_dt);
                    if r.is_pointer_button_down_on() && ctx.input(|i| i.pointer.primary_down()) {
                        self.camera += pan_offset(r.drag_delta(), self.camera.x_rad);
                        self.velocity = r.drag_delta() / dt;
                    } else {
                        const VELOCITY_DAMPING: f32 = 0.9999;
                        self.camera += pan_offset(self.velocity * dt, self.camera.x_rad);
                        self.velocity *= (1.0 - VELOCITY_DAMPING).powf(dt);
                    }
                    if self.velocity.length_sq() < 0.0001 {
                        self.velocity = Vec2::ZERO;
                    }
                    if r.contains_pointer()
                        && let Some(mouse_pos) = ctx.input(|i| i.pointer.latest_pos())
                    {
                        let mouse = mouse_pos - rect.center();
                        let zoom = ctx.input(|i| (i.smooth_scroll_delta.y / 300.0).exp());
                        self.camera += pan_offset(-mouse, self.camera.x_rad);
                        self.camera.x_rad /= zoom;
                        self.camera += pan_offset(mouse, self.camera.x_rad);
                    }
                }

                let camera_map = CameraMap::new(ui.max_rect(), self.camera);
                // ensure_pixel_safe for all pixels
                // if ctx.input(|i| i.key_down(Key::Space)) {
                //     for (_, pixel) in camera_map.pixels(self.stride) {
                //         self.tree.ensure_pixel_safe(pixel);
                //     }
                // }

                // // ensure_pixel_safe with time bound
                // if !ctx.input(|i| i.key_down(Key::Space)) {
                //     const MAX_TIME: Duration = Duration::from_millis(100);
                //     let start = Instant::now();
                //     let mut rng = rand::rng();
                //     let pixels = {
                //         let mut pixels = camera_map
                //             .pixels(self.stride)
                //             .map(|(_, pixel)| pixel)
                //             .filter(|pixel| !self.tree.contains_sample(*pixel))
                //             .collect::<Vec<_>>();
                //         pixels.shuffle(&mut rng);
                //         pixels
                //     };
                //     for pixel in pixels {
                //         if start.elapsed() > MAX_TIME {
                //             break;
                //         }
                //         if !self.tree.contains_sample(pixel) {
                //             self.tree.ensure_pixel_safe(pixel);
                //         }
                //     }
                // }

                // ensure_pixel_safe with time bound
                // but with a decreasing stride
                if !ctx.input(|i| i.key_down(Key::Space)) {
                    const MAX_TIME: Duration = Duration::from_millis(100);
                    let start = Instant::now();
                    let mut rng = rand::rng();

                    'outer: for stride_pow in
                        (self.stride.ilog2()..(ui.max_rect().width() as u32).ilog2()).rev()
                    {
                        let stride = 1 << stride_pow;

                        let pixels = {
                            let mut pixels = camera_map
                                .pixels(stride)
                                .map(|(_, _, pixel)| pixel)
                                .filter(|pixel| !self.tree.contains_sample(*pixel))
                                .collect::<Vec<_>>();
                            pixels.shuffle(&mut rng);
                            pixels
                        };
                        for pixel in pixels {
                            if start.elapsed() > MAX_TIME {
                                break 'outer;
                            }
                            if !self.tree.contains_sample(pixel) {
                                self.tree.ensure_pixel_safe(pixel);
                            }
                        }
                    }
                }

                // // draw the fractal
                // {
                //     let painter = ui.painter_at(ui.max_rect());

                //     painter.rect_filled(ui.max_rect(), 0.0, Color32::RED);

                //     // const STRIDE: u32 = 1;
                //     for (rect, pixel) in camera_map.pixels(self.stride) {
                //         let color = self.tree.color(pixel).build().unwrap_or(Color32::MAGENTA);
                //         // .expect("tree invariant not satisfied");

                //         painter.rect_filled(rect, 0.0, color);
                //     }
                // }

                // draw the fractal,
                // but instead of drawing error magenta,
                // draw pixels decreasing in stride
                {
                    let painter = ui.painter_at(ui.max_rect());

                    painter.rect_filled(ui.max_rect(), 0.0, Color32::RED);

                    // don't draw pixels that will be completely overdrawn in the future
                    let stride_pow_hi = {
                        || {
                            for stride_pow in
                                (self.stride.ilog2()..(ui.max_rect().width() as u32).ilog2()).rev()
                            {
                                let stride = 1 << stride_pow;
                                for (_, _, pixel) in camera_map.pixels(stride) {
                                    if !self.tree.contains_sample(pixel) {
                                        // +2 instead of +1 to fix a weird bug
                                        return stride_pow + 2;
                                    }
                                }
                            }
                            // idk why this need + 1
                            self.stride.ilog2() + 1
                        }
                    }();

                    // for stride_pow in (self.stride.ilog2()..=stride_pow_hi).rev() {
                    //     let stride = 1 << stride_pow;
                    //     for (_, rect, pixel) in camera_map.pixels(stride) {
                    //         if let Some(color) = self.tree.color_in_pixel(pixel).build() {
                    //             painter.rect_filled(rect, 0.0, color);
                    //         }
                    //     }
                    // }

                    // for stride_pow in [1] {
                    for stride_pow in (self.stride.ilog2()..=stride_pow_hi).rev() {
                        let stride = 1 << stride_pow;
                        assert_eq!(camera_map.rect.min, Pos2::ZERO);
                        // let pow_2_camera_map = CameraMap::new(
                        //     Rect {
                        //         min: Pos2::ZERO,
                        //         max: Pos2 {
                        //             x: (1 << (camera_map.rect.max.x as i32).ilog2()) as f32,
                        //             y: (1 << (camera_map.rect.max.y as i32).ilog2()) as f32,
                        //         },
                        //     },
                        //     camera_map.camera,
                        // );
                        // let colors = self.tree.color_in_pixels(
                        //     pow_2_camera_map.rect_to_window(pow_2_camera_map.rect),
                        //     (pow_2_camera_map.x_to_real(stride as f32)
                        //         - pow_2_camera_map.x_to_real(0.0))
                        //         / 2.0,
                        // );

                        let colors = self.tree.color_in_pixels(
                            camera_map.rect_to_window(camera_map.rect),
                            (camera_map.egui_to_cam_x(stride as f32) - camera_map.egui_to_cam_x(0.0)) / 2.0,
                            &camera_map,
                            stride,
                        );
                        for ((row, col), rect, pixel) in camera_map.pixels(stride) {
                            // for ((row, col), rect, pixel) in pow_2_camera_map.pixels(stride) {
                            // if let Some(color) = colors[row][col].clone().build() {
                            //     painter.rect_filled(rect, 0.0, color);
                            // }
                            // {
                            //     let rad = pixel.rad();
                            //     let actual = (camera_map.x_to_real(stride as f32)
                            //         - camera_map.x_to_real(0.0))
                            //         / 2.0;
                            //     assert!((rad - actual).abs() < 1e-6);
                            // }
                            if let Some(color) = colors
                                .get(row)
                                .and_then(|line| line.get(col))
                                .and_then(|c| c.clone().build())
                            {
                                // println!("here");
                                // let Some(oracle) = self.tree.color_in_pixel(pixel).build() else {
                                //     panic!()
                                // };
                                // assert_eq!(oracle, color);
                                painter.rect_filled(rect, 0.0, color);
                            } else if let Some(color) = self.tree.color_in_pixel(pixel).build() {
                                // painter.rect_filled(rect, 0.0, color);
                                painter.rect_filled(rect, 0.0, Color32::GREEN);
                                // painter.rect_stroke(
                                //     rect,
                                //     0.0,
                                //     egui::Stroke::new(0.5, Color32::MAGENTA),
                                //     egui::StrokeKind::Middle,
                                // );
                            }
                        }
                    }
                }

                // // draw sequence of nodes that contain the mouse
                // {
                //     if let Some(mouse_pos) = ctx.input(|i| i.pointer.latest_pos()) {
                //         fn draw_node(
                //             node: &Tree,
                //             depth: u32,
                //             painter: &egui::Painter,
                //             camera_map: &CameraMap,
                //             real: f32,
                //             imag: f32,
                //         ) {
                //             // println!("here");
                //             painter.rect_stroke(
                //                 camera_map.window_to_rect(node.window),
                //                 0.0,
                //                 egui::Stroke::new(
                //                     3.0,
                //                     // Color32::from_rgb(100, 100, 255u32.saturating_sub(5*depth) as u8),
                //                     // Color32::from_rgb(100, 100, {
                //                     //     let mut h = DefaultHasher::new();
                //                     //     depth.hash(&mut h);
                //                     //     h.finish() as u8
                //                     // }),
                //                     {
                //                         let mut h = DefaultHasher::new();
                //                         depth.hash(&mut h);
                //                         let hash = h.finish();
                //                         Color32::from_rgb(
                //                             (hash >> 24) as u8,
                //                             (hash >> 16) as u8,
                //                             (hash >> 8) as u8,
                //                         )
                //                     },
                //                 ),
                //                 egui::StrokeKind::Inside,
                //             );
                //             let Some(children) = &node.children else {
                //                 return;
                //             };
                //             for child in children {
                //                 if child.window.contains(real, imag) {
                //                     draw_node(child, depth + 1, painter, camera_map, real, imag);
                //                 }
                //             }
                //         }
                //         let (real, imag) = camera_map.pos_to_complex(mouse_pos);

                //         draw_node(
                //             &self.tree,
                //             0,
                //             &ui.painter_at(ui.max_rect()),
                //             &camera_map,
                //             real,
                //             imag,
                //         );
                //         // panic!();
                //     }
                // }

                // TODO: debug draw sequence of nodes that eventually have a child who's sample is inside the pixel the mouse is in

                // // debug coloring of how many samples are inside each pixel
                // {
                //     let painter = ui.painter_at(ui.max_rect());
                //     for (rect, pixel) in camera_map.pixels(self.stride) {
                //         // let color = self.tree.color(pixel).unwrap_or(Color32::MAGENTA);
                //         let count = self.tree.count_samples_strong(pixel);
                //         painter.rect_filled(
                //             rect,
                //             0.0,
                //             if count == 0 {
                //                 Color32::MAGENTA
                //             } else {
                //                 Color32::from_gray((count * 50).min(255) as u8)
                //             },
                //         );
                //     }
                // }

                // area is to allow the frame to be drawn on top of the fractal
                egui::Area::new(egui::Id::new("area"))
                    .constrain_to(ctx.screen_rect())
                    .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        // frame rate
                        {
                            let average_dt = self
                                .dts
                                .average()
                                .expect("we added one this frame so dts must be non-empty");
                            // ui.label(format!(
                            //     "    dt: {:08.04}\n1/dt: {:08.04}",
                            //     average_dt,
                            //     1.0 / average_dt,
                            // ));
                            ui.label(
                                RichText::new(format!(
                                    "    dt: {:08.04}\n1/dt: {:08.04}",
                                    average_dt,
                                    1.0 / average_dt,
                                ))
                                .background_color(Color32::BLACK),
                            );
                        }

                        // // view stuff
                        // {
                        //     ui.label(format!(
                        //         "center: {:12.09} + {:12.09}i\nreal_radius: {:12.09}",
                        //         self.camera.real_mid,
                        //         self.camera.imag_mid,
                        //         self.camera.real_rad,
                        //     ));
                        // }
                    });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_map() {
        let rect = Rect::from_min_max(Pos2::new(0.0, 30.0), Pos2::new(10.0, 50.0));
        let camera = Camera::new(1.0, 2.0, 3.0);
        let camera_map = CameraMap::new(rect, camera);

        assert_eq!(camera_map.camera.real_lo(), -2.0);
        assert_eq!(camera_map.camera.real_hi(), 4.0);
        assert_eq!(camera_map.imag_lo(), -4.0);
        assert_eq!(camera_map.imag_hi(), 8.0);

        assert!((0.0 - camera_map.real_to_x(-2.0)).abs() < 1e-4);
        assert!((10.0 - camera_map.real_to_x(4.0)).abs() < 1e-4);
        assert!((30.0 - camera_map.imag_to_y(8.0)).abs() < 1e-4);
        assert!((50.0 - camera_map.imag_to_y(-4.0)).abs() < 1e-4);

        for (p, c) in [
            (Pos2::new(0.0, 30.0), (-2.0, 8.0)),
            (Pos2::new(0.0, 50.0), (-2.0, -4.0)),
            (Pos2::new(10.0, 30.0), (4.0, 8.0)),
            (Pos2::new(10.0, 50.0), (4.0, -4.0)),
        ] {
            let c_actual = camera_map.egui_to_cam(p);
            assert!((c.0 - c_actual.0).abs() + (c.1 - c_actual.1).abs() < 1e-4);
            let p_actual = camera_map.complex_to_pos(c);
            assert!((p - p_actual).length() < 1e-4);
        }

        for pos in [
            Pos2::new(1.0, 30.0),
            Pos2::new(1.0, 50.0),
            Pos2::new(10.0, 30.0),
            Pos2::new(10.0, 50.0),
            Pos2::new(9.871, 38.635),
            Pos2::new(1.248, 45.656),
            Pos2::new(3.463, 48.559),
            Pos2::new(1.684, 32.323),
            Pos2::new(2.809, 31.250),
            Pos2::new(8.142, 36.146),
            Pos2::new(3.938, 48.579),
            Pos2::new(5.761, 42.575),
            Pos2::new(9.691, 42.933),
            Pos2::new(2.457, 30.097),
        ] {
            assert!(
                (pos - camera_map.complex_to_pos(camera_map.egui_to_cam(pos))).length() < 1e-4
            );
        }
        for c in [
            (-2.0, -1.0),
            (-2.0, 5.0),
            (4.0, -1.0),
            (4.0, 5.0),
            (-1.885, -0.978),
            (0.254, 0.793),
            (3.634, 3.274),
            (3.332, 1.716),
            (0.063, 3.933),
            (2.132, 1.927),
            (1.848, 4.781),
            (2.971, 4.047),
            (0.194, 2.966),
            (1.173, -0.435),
        ] {
            let actual = camera_map.egui_to_cam(camera_map.complex_to_pos(c));
            assert!((c.0 - actual.0).abs() + (c.1 - actual.1).abs() < 1e-4);
        }

        let window = Window::new(
            camera_map.camera.real_lo(),
            camera_map.camera.real_hi(),
            camera_map.imag_lo(),
            camera_map.imag_hi(),
        );
        assert_eq!(camera_map.rect, camera_map.window_to_rect(window));
        assert_eq!(window, camera_map.rect_to_window(rect));
    }
}
