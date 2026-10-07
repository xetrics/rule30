use std::time::{Duration, Instant};

use fontdue::Font;

use crate::color::Color;
use crate::drawing;

const FRAME_DURATIONS: [Duration; 3] = [
    Duration::from_millis(1),
    Duration::from_millis(10),
    Duration::from_millis(100),
];
const DRAW_ORIGIN: (usize, usize) = (20, 10); // (w, 0) + (-x,y)
const MAX_STEPS_PER_FRAME: u8 = 5;

pub struct State {
    width: usize,
    height: usize,

    fd_idx: usize,
    paused: bool,

    last_time: Instant,
    frame_timer: Duration,
    pending_steps: u8,
    steps: u64,
    font: Font,
}

impl State {
    pub fn new(width: usize, height: usize) -> Self {
        let font = drawing::load_font();

        Self {
            fd_idx: FRAME_DURATIONS.len() - 1,
            paused: true,
            width,
            height,
            last_time: Instant::now(),
            frame_timer: Duration::ZERO,
            pending_steps: 0,
            steps: 0,
            font,
        }
    }

    pub fn frame_duration(&self) -> Duration {
        FRAME_DURATIONS[self.fd_idx]
    }

    pub fn inc_frame_duration(&mut self) {
        self.fd_idx = (self.fd_idx + 1).min(FRAME_DURATIONS.len() - 1);
    }

    pub fn dec_frame_duration(&mut self) {
        self.fd_idx = self.fd_idx.saturating_sub(1);
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused
    }

    pub fn step_once(&mut self) {
        self.pending_steps = self.pending_steps.saturating_add(1);
    }

    // returns number of times simulation should be ticked this frame, if at all
    pub fn tick(&mut self) -> u8 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_time);
        self.last_time = now;

        if self.paused {
            self.frame_timer = Duration::ZERO;
            return std::mem::take(&mut self.pending_steps);
        }

        let step = self.frame_duration();

        self.frame_timer += delta;

        let mut sims = 0;
        while self.frame_timer >= step && sims < MAX_STEPS_PER_FRAME {
            self.frame_timer -= step;
            sims += 1;
        }

        if sims == MAX_STEPS_PER_FRAME {
            self.frame_timer = Duration::ZERO;
        }

        self.steps = self.steps.saturating_add(sims as u64);
        sims
    }

    pub fn draw(&self, screen: &mut [u8]) {
        let origin_x = self.width - DRAW_ORIGIN.0;
        let origin_y = DRAW_ORIGIN.1;

        // paused
        drawing::draw_square(
            screen,
            self.width,
            self.height,
            origin_x,
            origin_y,
            5,
            match self.paused {
                true => Color::RED,
                false => Color::GREEN,
            },
        );

        // speed
        let origin_y = origin_y + 10;
        let squares = FRAME_DURATIONS.len();
        let lit = squares - self.fd_idx; // slowest = 1, fastest = all
        for dx in 0..squares {
            let x = origin_x - (squares - 1 - dx) * 7;
            let color = if dx < lit {
                Color::PRIMARY
            } else {
                Color::BACKGROUND_ALT
            };
            drawing::draw_square(screen, self.width, self.height, x, origin_y, 5, color);
        }

        // sims
        let steps: String = self.steps.to_string();
        let origin_y = origin_y + 10;
        let text_x = (origin_x + 5).saturating_sub(drawing::text_width(&self.font, &steps, 16.0));
        drawing::draw_text(
            screen,
            self.width,
            self.height,
            &self.font,
            &steps,
            16.0,
            text_x,
            origin_y,
            Color::WHITE,
        );
    }
}
