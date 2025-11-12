use pixels::{Pixels, SurfaceTexture};
use screenshots::Screen;
use winit::{
    event::{Event, WindowEvent, MouseScrollDelta, TouchPhase, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;

struct ViewState {
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
    drag_start: Option<(f32, f32)>,
    last_cursor_pos: (f32, f32),
}

impl ViewState {
    fn new() -> Self {
        Self {
            zoom: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
            drag_start: None,
            last_cursor_pos: (0.0, 0.0),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let screens = Screen::all()?;
    let screen = screens.first().ok_or("No screens found")?;
    
    let screenshot = screen.capture()?;
    let screenshot_width = screenshot.width();
    let screenshot_height = screenshot.height();
    
    let img_buffer = image::RgbaImage::from_raw(
        screenshot_width,
        screenshot_height,
        screenshot.to_vec(),
    )
    .ok_or("Failed to create image buffer")?;

    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
        .with_title("Screenshot Viewer")
        .with_decorations(false)
        .build(&event_loop)?;

    #[cfg(target_os = "macos")]
    window.set_simple_fullscreen(true);

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture)?;

    let mut view_state = ViewState::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(new_size) => {
                    if new_size.width > 0 && new_size.height > 0 {
                        let _ = pixels.resize_surface(new_size.width, new_size.height);
                        let _ = pixels.resize_buffer(new_size.width, new_size.height);
                        window.request_redraw();
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    view_state.last_cursor_pos = (position.x as f32, position.y as f32);
                    
                    if let Some((start_x, start_y)) = view_state.drag_start {
                        let dx = position.x as f32 - start_x;
                        let dy = position.y as f32 - start_y;
                        view_state.offset_x += dx;
                        view_state.offset_y += dy;
                        view_state.drag_start = Some((position.x as f32, position.y as f32));
                        window.request_redraw();
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    match state {
                        winit::event::ElementState::Pressed => {
                            if button == winit::event::MouseButton::Left {
                                view_state.drag_start = Some(view_state.last_cursor_pos);
                            }
                        }
                        winit::event::ElementState::Released => {
                            view_state.drag_start = None;
                        }
                    }
                }
                WindowEvent::MouseWheel { delta, phase, .. } => {
                    if phase == TouchPhase::Moved || phase == TouchPhase::Started {
                        match delta {
                            MouseScrollDelta::LineDelta(_, y) => {
                                let zoom_factor = 1.0 + (y * 0.1);
                                view_state.zoom *= zoom_factor;
                                view_state.zoom = view_state.zoom.max(0.1).min(10.0);
                                
                                let cursor_x = view_state.last_cursor_pos.0;
                                let cursor_y = view_state.last_cursor_pos.1;
                                view_state.offset_x = cursor_x - (cursor_x - view_state.offset_x) * zoom_factor;
                                view_state.offset_y = cursor_y - (cursor_y - view_state.offset_y) * zoom_factor;
                                
                                window.request_redraw();
                            }
                            MouseScrollDelta::PixelDelta(pos) => {
                                let zoom_delta = pos.y as f32;
                                if zoom_delta.abs() > 0.1 {
                                    let zoom_factor = 1.0 + (zoom_delta * 0.005);
                                    view_state.zoom *= zoom_factor;
                                    view_state.zoom = view_state.zoom.max(0.1).min(10.0);
                                    
                                    let cursor_x = view_state.last_cursor_pos.0;
                                    let cursor_y = view_state.last_cursor_pos.1;
                                    view_state.offset_x = cursor_x - (cursor_x - view_state.offset_x) * zoom_factor;
                                    view_state.offset_y = cursor_y - (cursor_y - view_state.offset_y) * zoom_factor;
                                }
                                
                                window.request_redraw();
                            }
                        }
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        if let Some(VirtualKeyCode::Q) = input.virtual_keycode {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                render(
                    pixels.frame_mut(),
                    &img_buffer,
                    &view_state,
                    window_size.width,
                    window_size.height,
                );
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });
}

fn render(
    frame: &mut [u8],
    screenshot: &image::RgbaImage,
    view_state: &ViewState,
    window_width: u32,
    window_height: u32,
) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = 0;
        pixel[1] = 0;
        pixel[2] = 0;
        pixel[3] = 255;
    }

    let screenshot_width = screenshot.width();
    let screenshot_height = screenshot.height();

    for y in 0..window_height {
        for x in 0..window_width {
            let src_x_f = (x as f32 - view_state.offset_x) / view_state.zoom;
            let src_y_f = (y as f32 - view_state.offset_y) / view_state.zoom;

            if src_x_f >= 0.0 && src_y_f >= 0.0 {
                let src_x = src_x_f as u32;
                let src_y = src_y_f as u32;

                if src_x < screenshot_width && src_y < screenshot_height {
                    let pixel = screenshot.get_pixel(src_x, src_y);
                    let offset = (y * window_width + x) as usize * 4;

                    if offset + 3 < frame.len() {
                        frame[offset] = pixel[0];
                        frame[offset + 1] = pixel[1];
                        frame[offset + 2] = pixel[2];
                        frame[offset + 3] = pixel[3];
                    }
                }
            }
        }
    }
}

