//! live_window — 使用 winit 0.28 + pixels 0.12 開啟真實 OS 視窗。
//!
//! 執行：  cargo run --example live_window
//!
//! 操作：
//!   ESC / 關閉視窗 → 結束
//!   方向鍵         → 加速彈跳球
//!   空白鍵         → 切換漸層

use minisdl::{
    color::Color,
    graphics::Canvas,
    rect::{Point, Rect},
    window::Window,
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const W: u32 = 800;
const H: u32 = 600;

struct AppState {
    angle:     f32,
    ball_x:    f32,
    ball_y:    f32,
    ball_dx:   f32,
    ball_dy:   f32,
    flip_grad: bool,
    frame:     u64,
}

impl AppState {
    fn new() -> Self {
        Self {
            angle: 0.0,
            ball_x: 400.0, ball_y: 300.0,
            ball_dx: 2.5,  ball_dy: 1.8,
            flip_grad: false,
            frame: 0,
        }
    }

    fn update(&mut self) {
        self.angle   = (self.angle + 1.2) % 360.0;
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;
        if self.ball_x < 30.0 || self.ball_x > W as f32 - 30.0 { self.ball_dx = -self.ball_dx; }
        if self.ball_y < 30.0 || self.ball_y > H as f32 - 30.0 { self.ball_dy = -self.ball_dy; }
        self.frame += 1;
    }

    fn draw(&self, window: &mut Window) {
        let mut canvas = window.canvas();

        // 背景漸層
        let (top, bot) = if self.flip_grad {
            (Color::rgb(10, 30, 60), Color::rgb(60, 10, 40))
        } else {
            (Color::rgb(15, 10, 30), Color::rgb(40, 20, 60))
        };
        canvas.fill_gradient_v(Rect::new(0, 0, W, H), top, bot).unwrap();

        // 軌道
        canvas.set_draw_color(Color::rgba(255, 255, 255, 40));
        canvas.draw_circle(W as i32 / 2, H as i32 / 2, 150).unwrap();

        // 軌道球
        let ox = (W as f32 / 2.0 + self.angle.to_radians().cos() * 150.0) as i32;
        let oy = (H as f32 / 2.0 + self.angle.to_radians().sin() * 150.0) as i32;
        canvas.set_draw_color(Color::CYAN);
        canvas.fill_circle(ox, oy, 14).unwrap();
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_circle(ox, oy, 14).unwrap();

        // 彈跳球
        canvas.set_draw_color(Color::rgb(255, 120, 40));
        canvas.fill_circle(self.ball_x as i32, self.ball_y as i32, 22).unwrap();
        canvas.set_draw_color(Color::YELLOW);
        canvas.draw_circle(self.ball_x as i32, self.ball_y as i32, 22).unwrap();

        // 中央面板
        canvas.set_draw_color(Color::rgba(0, 0, 0, 120));
        canvas.fill_rounded_rect(Rect::new(250, 260, 300, 80), 14).unwrap();
        canvas.set_draw_color(Color::rgba(255, 255, 255, 60));
        canvas.draw_rounded_rect(Rect::new(250, 260, 300, 80), 14).unwrap();

        // Bézier 裝飾
        let t = self.frame as f32 * 0.02;
        canvas.set_draw_color(Color::rgba(80, 220, 200, 180));
        canvas.draw_bezier_cubic(
            Point::new(50, 550),
            Point::new(200 + (t.sin() * 80.0) as i32, 400),
            Point::new(600 + (t.cos() * 80.0) as i32, 500),
            Point::new(750, 450),
            80,
        ).unwrap();

        // 底部彩虹條
        canvas.fill_gradient_h(Rect::new(0, H as i32 - 6, W, 6), Color::CYAN, Color::MAGENTA).unwrap();

        canvas.present().unwrap();
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let winit_window = WindowBuilder::new()
        .with_title("MiniSDL — Live Window  [ESC 結束 | 方向鍵加速 | 空白鍵換漸層]")
        .with_inner_size(LogicalSize::new(W, H))
        .with_resizable(false)
        .build(&event_loop)
        .expect("視窗建立失敗");

    let mut pixels = {
        let size = winit_window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, &winit_window);
        Pixels::new(W, H, surface).expect("Pixels 初始化失敗")
    };

    let mut mini_window = Window::new("live", W, H).unwrap();
    let mut state = AppState::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(key),
                        ..
                    },
                    ..
                },
                ..
            } => match key {
                VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                VirtualKeyCode::Space  => state.flip_grad = !state.flip_grad,
                VirtualKeyCode::Left   => state.ball_dx = -(state.ball_dx.abs() + 0.5),
                VirtualKeyCode::Right  => state.ball_dx =   state.ball_dx.abs() + 0.5,
                VirtualKeyCode::Up     => state.ball_dy = -(state.ball_dy.abs() + 0.5),
                VirtualKeyCode::Down   => state.ball_dy =   state.ball_dy.abs() + 0.5,
                _ => {}
            },

            Event::MainEventsCleared => {
                state.update();
                state.draw(&mut mini_window);

                // 把 MiniSDL RGBA framebuffer 複製給 pixels
                pixels.frame_mut().copy_from_slice(mini_window.pixels());

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }
    });
}
