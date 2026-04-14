use std::time::Duration;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinHandle;
use tokio::time::interval;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Render,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    pub rx: UnboundedReceiver<Event>,
    task: JoinHandle<()>,
    cancellation_token: CancellationToken,
}

impl EventHandler {
    pub fn new(tick_rate: f64, render_rate: f64) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();
        let ct = cancellation_token.clone();

        let task = tokio::spawn(async move {
            let mut event_stream = crossterm::event::EventStream::new();
            let mut tick_interval = interval(Duration::from_secs_f64(1.0 / tick_rate));
            let mut render_interval = interval(Duration::from_secs_f64(1.0 / render_rate));

            loop {
                let event = tokio::select! {
                    _ = ct.cancelled() => break,
                    _ = tick_interval.tick() => Event::Tick,
                    _ = render_interval.tick() => Event::Render,
                    crossterm_event = event_stream.next().fuse() => match crossterm_event {
                        Some(Ok(CrosstermEvent::Key(key))) if key.kind == KeyEventKind::Press => Event::Key(key),
                        Some(Ok(CrosstermEvent::Mouse(mouse))) => Event::Mouse(mouse),
                        Some(Ok(CrosstermEvent::Resize(x, y))) => Event::Resize(x, y),
                        _ => continue,
                    },
                };

                if tx.send(event).is_err() {
                    break;
                }
            }
        });

        Self { rx, task, cancellation_token }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }

    pub fn stop(&self) {
        self.cancellation_token.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));
            counter += 1;
            if counter > 50 {
                self.task.abort();
            }
            if counter > 200 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_handler_creation() {
        let eh = EventHandler::new(4.0, 30.0);
        assert!(!eh.task.is_finished());
        eh.stop();
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        assert!(eh.task.is_finished());
    }
}
