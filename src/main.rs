mod cell;
mod color;
mod drawing;
mod grid;
mod state;

use crate::{grid::Grid, state::State};
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::{Fullscreen, Window},
};
use winit_input_helper::WinitInputHelper;

const CELL_SIZE: u32 = 4;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    #[allow(deprecated)]
    let window = {
        Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Rule 30")
                        .with_fullscreen(Some(Fullscreen::Borderless(None))),
                )
                .unwrap(),
        )
    };

    let monitor = window
        .current_monitor()
        .or_else(|| window.primary_monitor())
        .or_else(|| window.available_monitors().next())
        .expect("no monitor found");
    let screen: PhysicalSize<u32> = monitor.size();

    let grid_w = screen.width / CELL_SIZE;
    let grid_h = screen.height / CELL_SIZE;

    println!(
        "screen {}x{}, grid {}x{}",
        screen.width, screen.height, grid_w, grid_h
    );

    let mut pixels = {
        let surface = SurfaceTexture::new(screen.width, screen.height, &window);
        Pixels::new(grid_w, grid_h, surface).unwrap()
    };

    let mut state = State::new(grid_w as usize, grid_h as usize);
    let mut grid = Grid::new(grid_w as usize, grid_h as usize);

    #[allow(deprecated)]
    let res = event_loop.run(|event, elwt| match &event {
        Event::NewEvents(_) => input.step(),

        Event::WindowEvent { event, .. } => {
            if input.process_window_event(event) {
                grid.draw(pixels.frame_mut());
                state.draw(pixels.frame_mut());
                pixels.render().unwrap();
            }
        }

        Event::DeviceEvent { event, .. } => input.process_device_event(event),

        Event::AboutToWait => {
            input.end_step();

            // close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() || input.destroyed() {
                elwt.exit();
                return;
            }

            // resize
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height).unwrap();
                let grid_w = (size.width / CELL_SIZE) as usize;
                let grid_h = (size.height / CELL_SIZE) as usize;
                grid = Grid::new(grid_w, grid_h);
                state = State::new(grid_w, grid_h);
            }

            // pause/play
            if input.key_pressed(KeyCode::KeyP) {
                state.toggle_pause();
            }

            // speed up/down
            if input.key_pressed(KeyCode::ArrowUp) {
                state.dec_frame_duration();
            }
            if input.key_pressed(KeyCode::ArrowDown) {
                state.inc_frame_duration();
            }

            // stepping
            if input.key_pressed_os(KeyCode::Space) {
                if state.paused() {
                    state.step_once();
                } else {
                    state.pause();
                }
            }

            // reset
            if input.key_pressed(KeyCode::KeyR) {
                grid.reset();
                state.pause();
            }

            // update timings
            let sims = state.tick();

            // update state and request redraw
            for _ in 0..sims {
                grid.step();
            }
            window.request_redraw();
        }

        _ => {}
    });

    res.unwrap();
}
