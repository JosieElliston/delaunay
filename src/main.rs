mod point;
mod quartic;
mod triangulation;
mod utils;

use eframe::egui::{self, Color32, Key, Pos2, Rect, RichText, Vec2};

use crate::point::*;
use crate::utils::*;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let native_options = eframe::NativeOptions::default();

    let _ = eframe::run_native(
        "delaunay",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
    // println!("here");
}

#[derive(Debug, Clone, Copy)]
struct Camera {
    x_mid: Coord,
    y_mid: Coord,
    x_rad: Coord,
}
impl Camera {
    fn new(x_mid: Coord, y_mid: Coord, x_rad: Coord) -> Self {
        assert!(x_rad > 0.0);
        Self {
            x_mid,
            y_mid,
            x_rad,
        }
    }

    fn x_lo(self) -> Coord {
        self.x_mid - self.x_rad
    }

    fn x_hi(self) -> Coord {
        self.x_mid + self.x_rad
    }

    // fn imag_lo(self) -> Coord {}

    // fn imag_hi(self) -> Coord {}
}
impl std::ops::AddAssign<(f32, f32)> for Camera {
    fn add_assign(&mut self, rhs: (f32, f32)) {
        self.x_mid += rhs.0 as Coord;
        self.y_mid += rhs.1 as Coord;
    }
}
impl std::ops::SubAssign<(f32, f32)> for Camera {
    fn sub_assign(&mut self, rhs: (f32, f32)) {
        self.x_mid -= rhs.0 as Coord;
        self.y_mid -= rhs.1 as Coord;
    }
}

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

    fn y_rad(&self) -> Coord {
        self.camera.x_rad * self.rect.height() as Coord / self.rect.width() as Coord
    }

    fn y_lo(&self) -> Coord {
        self.camera.y_mid - self.y_rad()
    }

    fn y_hi(&self) -> Coord {
        self.camera.y_mid + self.y_rad()
    }

    fn egui_to_cam_x(&self, x: f32) -> Coord {
        lerp_coord(
            self.camera.x_lo(),
            self.camera.x_hi(),
            inv_lerp_f32(self.rect.min.x, self.rect.max.x, x) as Coord,
        )
    }

    fn egui_to_cam_y(&self, y: f32) -> Coord {
        lerp_coord(
            self.y_lo(),
            self.y_hi(),
            1.0 - inv_lerp_f32(self.rect.min.y, self.rect.max.y, y) as Coord,
        )
    }

    fn egui_to_cam(&self, pos: Pos2) -> Point {
        Point::new(self.egui_to_cam_x(pos.x), self.egui_to_cam_y(pos.y))
    }

    fn cam_to_egui_x(&self, x: Coord) -> f32 {
        lerp_f32(
            self.rect.min.x,
            self.rect.max.x,
            inv_lerp_coord(self.camera.x_lo(), self.camera.x_hi(), x) as f32,
        )
    }

    fn cam_to_egui_y(&self, y: Coord) -> f32 {
        lerp_f32(
            self.rect.min.y,
            self.rect.max.y,
            1.0 - inv_lerp_coord(self.y_lo(), self.y_hi(), y) as f32,
        )
    }

    fn cam_to_egui(&self, point: Point) -> Pos2 {
        Pos2::new(self.cam_to_egui_x(point.x()), self.cam_to_egui_y(point.y()))
    }

    fn pan(&mut self, pan_vec: Pos2) {
        todo!()
    }

    // fn window_to_rect(&self, window: impl Into<Window>) -> Rect {
    //     let window = window.into();
    //     Rect {
    //         min: self.complex_to_pos((window.real_lo, window.imag_hi)),
    //         max: self.complex_to_pos((window.real_hi, window.imag_lo)),
    //     }
    // }

    // fn rect_to_window(&self, rect: Rect) -> Window {
    //     Window::new(
    //         self.egui_to_cam_x(rect.min.x),
    //         self.egui_to_cam_x(rect.max.x),
    //         self.egui_to_cam_y(rect.max.y),
    //         self.egui_to_cam_y(rect.min.y),
    //     )
    // }
}

struct App {
    camera: Camera,
    velocity: Vec2,
    dts: egui::util::History<f32>,
    points: Vec<Point>,
    drag_point: Option<usize>,
}
impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            camera: Camera::new(0.5, 0.5, 0.5),
            velocity: Vec2::ZERO,
            dts: egui::util::History::new(1..256, 1.0),
            points: [
                (0.695, 0.5075),
                (0.60425, 0.59325),
                (0.51, 0.7335),
                (0.6975, 0.72),
                (0.8855, 0.90325),
                (0.80825, 0.648),
                (0.68925, 0.7925),
                (0.1325, 0.855),
                (0.959, 0.0125),
                (1.015, 0.516),
            ]
            .map(Point::from)
            .to_vec(),
            drag_point: None,
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

                let camera_map = CameraMap::new(ui.max_rect(), self.camera);

                // this is in egui coords
                const HIGHLIGHT_DIST: f32 = 10.0;
                // the point the mouse is closest to and within threshold distance
                let highlighted_point = {
                    if let Some(drag_point) = self.drag_point {
                        Some(drag_point)
                    } else {
                        // TODO: quadtree lmao
                        ctx.input(|i| i.pointer.hover_pos()).and_then(|mouse_pos| {
                            self.points
                                .iter()
                                .enumerate()
                                .filter_map(|(i, &point)| {
                                    let dist = (camera_map.cam_to_egui(point) - mouse_pos).length();
                                    if dist < HIGHLIGHT_DIST {
                                        Some((i, dist))
                                    } else {
                                        None
                                    }
                                })
                                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                                .map(|(i, _)| i)
                        })
                    }
                };

                // panning stuff
                {
                    let rect = ui.available_rect_before_wrap();
                    let r = ui.allocate_rect(rect, egui::Sense::click_and_drag());

                    let pan_offset = |pan_vec: Vec2, real_rad: f32| -> (f32, f32) {
                        (
                            2.0 * pan_vec.x / rect.size().x * real_rad,
                            -2.0 * pan_vec.y * real_rad / rect.size().x,
                        )
                    };

                    let dt = ctx.input(|input_state| input_state.stable_dt);
                    let mouse_down =
                        r.is_pointer_button_down_on() && ctx.input(|i| i.pointer.primary_down());

                    if !mouse_down {
                        self.drag_point = None;
                    }
                    if let Some(highlighted_point) = highlighted_point
                        && ctx.input(|i| i.pointer.primary_pressed())
                    {
                        self.drag_point = Some(highlighted_point);
                    }

                    // pan / drag
                    if mouse_down {
                        if let Some(drag_point) = self.drag_point {
                            // self.points[drag_point] +=
                            //     pan_offset(r.drag_delta(), self.camera.x_rad as _).into();
                            if let Some(mouse_pos) = ctx.input(|i| i.pointer.latest_pos()) {
                                let cam_pos = camera_map.egui_to_cam(mouse_pos);
                                self.points[drag_point] = cam_pos;
                            }
                        } else if r.drag_delta() != Vec2::ZERO {
                            self.camera -= pan_offset(r.drag_delta(), self.camera.x_rad as _);
                            self.velocity = r.drag_delta() / dt;
                        }
                    } else {
                        const VELOCITY_DAMPING: f32 = 16.0;
                        // self.velocity *= (1.0 - VELOCITY_DAMPING).powf(dt);
                        self.velocity *= 1.0 / (1.0 + (dt * VELOCITY_DAMPING));
                        self.camera -= pan_offset(self.velocity * dt, self.camera.x_rad as _);
                    }
                    if self.velocity.length_sq() < 0.0001 {
                        self.velocity = Vec2::ZERO;
                    }

                    // zoom
                    if r.contains_pointer()
                        && let Some(mouse_pos) = ctx.input(|i| i.pointer.latest_pos())
                    {
                        let mouse = mouse_pos - rect.center();
                        let zoom = ctx.input(|i| (i.smooth_scroll_delta.y / 300.0).exp());
                        self.camera -= pan_offset(-mouse, self.camera.x_rad as _);
                        self.camera.x_rad /= zoom as Coord;
                        self.camera -= pan_offset(mouse, self.camera.x_rad as _);
                    }
                }
                let camera_map = CameraMap::new(ui.max_rect(), self.camera);

                // drawing
                {
                    let painter = ui.painter_at(ui.max_rect());

                    // draw debug ~quadtree lines
                    {
                        let draw = |mid: Point, rad: Coord| {
                            const PRIMARY_COLOR: Color32 = Color32::from_rgb(10, 10, 255);
                            const SECONDARY_COLOR: Color32 = Color32::from_rgb(100, 100, 200);

                            let ul =
                                camera_map.cam_to_egui(Point::new(mid.x() - rad, mid.y() + rad));
                            let ur =
                                camera_map.cam_to_egui(Point::new(mid.x() + rad, mid.y() + rad));
                            let dl =
                                camera_map.cam_to_egui(Point::new(mid.x() - rad, mid.y() - rad));
                            let dr =
                                camera_map.cam_to_egui(Point::new(mid.x() + rad, mid.y() - rad));

                            painter.line_segment([ul, ur], (1.0, PRIMARY_COLOR));
                            painter.line_segment([ur, dr], (1.0, PRIMARY_COLOR));
                            painter.line_segment([dr, dl], (1.0, PRIMARY_COLOR));
                            painter.line_segment([dl, ul], (1.0, PRIMARY_COLOR));

                            let um = camera_map.cam_to_egui(Point::new(mid.x(), mid.y() + rad));
                            let dm = camera_map.cam_to_egui(Point::new(mid.x(), mid.y() - rad));
                            let lm = camera_map.cam_to_egui(Point::new(mid.x() - rad, mid.y()));
                            let rm = camera_map.cam_to_egui(Point::new(mid.x() + rad, mid.y()));

                            painter.line_segment([um, dm], (1.0, SECONDARY_COLOR));
                            painter.line_segment([lm, rm], (1.0, SECONDARY_COLOR));
                        };

                        draw(Point::new(1.0 / 2.0, 1.0 / 2.0), 1.0 / 2.0);
                        draw(Point::new(3.0 / 4.0, 3.0 / 4.0), 1.0 / 4.0);
                        draw(Point::new(5.0 / 8.0, 5.0 / 8.0), 1.0 / 8.0);
                    }

                    // draw triangulation
                    {
                        const COLOR: Color32 = Color32::from_rgb(150, 150, 150);
                        let triangles = quartic::triangulate(&self.points);
                        for (i, j, k) in triangles {
                            let a = camera_map.cam_to_egui(self.points[i]);
                            let b = camera_map.cam_to_egui(self.points[j]);
                            let c = camera_map.cam_to_egui(self.points[k]);
                            painter.line_segment([a, b], (1.0, COLOR));
                            painter.line_segment([b, c], (1.0, COLOR));
                            painter.line_segment([c, a], (1.0, COLOR));
                        }
                    }

                    // draw points
                    for (i, &point) in self.points.iter().enumerate() {
                        if Some(i) == highlighted_point {
                            painter.circle_filled(
                                camera_map.cam_to_egui(point),
                                HIGHLIGHT_DIST,
                                Color32::from_rgb(200, 200, 200),
                            );
                        } else {
                            painter.circle_filled(
                                camera_map.cam_to_egui(point),
                                5.0,
                                Color32::from_rgb(200, 200, 200),
                            );
                        }
                    }
                }

                // area is to allow the frame to be drawn on top
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

        assert_eq!(camera_map.camera.x_lo(), -2.0);
        assert_eq!(camera_map.camera.x_hi(), 4.0);
        assert_eq!(camera_map.y_lo(), -4.0);
        assert_eq!(camera_map.y_hi(), 8.0);

        assert!((0.0 - camera_map.cam_to_egui_x(-2.0)).abs() < 1e-4);
        assert!((10.0 - camera_map.cam_to_egui_x(4.0)).abs() < 1e-4);
        assert!((30.0 - camera_map.cam_to_egui_y(8.0)).abs() < 1e-4);
        assert!((50.0 - camera_map.cam_to_egui_y(-4.0)).abs() < 1e-4);

        for (p, c) in [
            (Pos2::new(0.0, 30.0), Point::new(-2.0, 8.0)),
            (Pos2::new(0.0, 50.0), Point::new(-2.0, -4.0)),
            (Pos2::new(10.0, 30.0), Point::new(4.0, 8.0)),
            (Pos2::new(10.0, 50.0), Point::new(4.0, -4.0)),
        ] {
            let c_actual = camera_map.egui_to_cam(p);
            assert!((c.x() - c_actual.x()).abs() + (c.y() - c_actual.y()).abs() < 1e-4);
            let p_actual = camera_map.cam_to_egui(c);
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
            assert!((pos - camera_map.cam_to_egui(camera_map.egui_to_cam(pos))).length() < 1e-4);
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
        ]
        .map(Point::from)
        {
            let actual = camera_map.egui_to_cam(camera_map.cam_to_egui(c));
            assert!((c.x() - actual.x()).abs() + (c.y() - actual.y()).abs() < 1e-4);
        }

        // let window = Window::new(
        //     camera_map.camera.x_lo(),
        //     camera_map.camera.x_hi(),
        //     camera_map.y_lo(),
        //     camera_map.y_hi(),
        // );
        // assert_eq!(camera_map.rect, camera_map.window_to_rect(window));
        // assert_eq!(window, camera_map.rect_to_window(rect));
    }
}
