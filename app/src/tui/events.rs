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
                // The default tick_rate is 1 FPS, which means that the terminal
                // will wait for an event for 1 second before sending a Tick
                // event itself.
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if poll(timeout).unwrap() {
                    match read().unwrap() {
                        crossterm::event::Event::Key(key) => {
                            tx.send(Event::Input(key.code)).unwrap()
                        },
                        crossterm::event::Event::Resize(_, _) => {
                            tx.send(Event::Tick).unwrap()
                        },
                        _ => {},
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
        Self::new(Duration::from_millis(1000))
    }
}
