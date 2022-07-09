use std::time::Duration;

use crossterm::event::{poll, read, KeyCode};
use tokio::{
    spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver},
    time::Instant,
};

#[derive(Debug)]
pub(crate) enum Event<I> {
    Input(I),
    Tick,
}

pub(crate) struct EventLoop {
    pub rx: UnboundedReceiver<Event<KeyCode>>,
}

impl EventLoop {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = unbounded_channel();

        spawn(async move {
            let mut last_tick = Instant::now();

            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if poll(timeout).unwrap() {
                    if let crossterm::event::Event::Key(key) = read().unwrap() {
                        tx.send(Event::Input(key.code)).unwrap();
                    }
                }

                if tx.is_closed() {
                    break;
                }

                if last_tick.elapsed() >= tick_rate {
                    tx.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        Self { rx }
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        // 120 FPS
        Self::new(Duration::from_millis(1000 / 120))
    }
}
