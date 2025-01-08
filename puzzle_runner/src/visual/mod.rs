//! Helpers for visualizations.
#![cfg(feature = "visual")]

use std::{
    sync::{mpsc, MutexGuard},
    thread::{self, JoinHandle},
};

use font_kit::{family_name::FamilyName, font::Font, properties::Properties, source::SystemSource};
use minifb::{Key, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};

const WIDTH: usize = 1128;
const HEIGHT: usize = 1128;

/// Information available for visualizations.
pub struct Config {
    pub font: Font,
}

/// Trait for the per-solution implementation that takes the visualization info and converts it into a graphics.
pub trait Visual {
    type Info;

    /// Process new information from the solution.
    fn update<I>(&mut self, info: I)
    where
        I: Iterator<Item = Self::Info>;

    /// Render the current state.
    fn draw(&self, dt: &mut DrawTarget, config: &Config);
}

/// Trait to allow info-agnostic rendering.
pub trait Renderable {
    fn draw(&mut self, dt: &mut DrawTarget, config: &Config);
}

/// A [`Visual`] + a channel delivering info for the visual.
pub struct CompleteVisual<V>
where
    V: Visual + 'static,
{
    visual: V,
    rx: MutexGuard<'static, mpsc::Receiver<V::Info>>,
}
impl<V> CompleteVisual<V>
where
    V: Visual + 'static,
{
    pub fn new(visual: V, rx: MutexGuard<'static, mpsc::Receiver<V::Info>>) -> Self {
        Self { visual, rx }
    }
}
impl<V> Renderable for CompleteVisual<V>
where
    V: Visual + 'static,
{
    fn draw(&mut self, dt: &mut DrawTarget, config: &Config) {
        self.visual.update(self.rx.try_iter());
        self.visual.draw(dt, config);
    }
}

/// Marker for derive macro that allows converting [`Visual`] into [`Renderable`].
pub trait ToRenderable: Into<Box<dyn Renderable>> {}
/// Derive macro for [`ToRenderable`].
pub use puzzle_runner_derive::ToRenderable;

pub fn spawn_window<F>(creator: F) -> JoinHandle<()>
where
    F: (FnOnce() -> Box<dyn Renderable>) + Send + 'static,
{
    thread::spawn(|| {
        window_main(creator());
    })
}

fn window_main(mut renderable: Box<dyn Renderable>) {
    let mut window = Window::new(
        "Advent of Code",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let font = SystemSource::new()
        .select_best_match(&[FamilyName::Monospace], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let config = Config { font };

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    dt.clear(SolidSource::from_unpremultiplied_argb(
        0xff, 0xff, 0xff, 0xff,
    ));
    dt.draw_text(
        &config.font,
        32.0,
        "Press S to start the visualization.",
        Point::new(20.0, 50.0),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xff, 0x00, 0x00, 0x00,
        )),
        &DrawOptions::default(),
    );
    dt.draw_text(
        &config.font,
        32.0,
        "Press Q (at any time) to quit the visualization.",
        Point::new(20.0, 90.0),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            0xff, 0x00, 0x00, 0x00,
        )),
        &DrawOptions::default(),
    );

    let mut started = false;
    loop {
        if window.is_key_down(Key::Q) {
            break;
        } else if window.is_key_down(Key::S) {
            started = true;
        }

        if started {
            dt.clear(SolidSource::from_unpremultiplied_argb(
                0xff, 0xff, 0xff, 0xff,
            ));
            renderable.draw(&mut dt, &config);
        }

        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();
    }
}
