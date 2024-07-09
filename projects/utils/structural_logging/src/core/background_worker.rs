use std::{
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    thread::{self, JoinHandle}};

use crate::{models::LogDataHolder, traits::StructuralLogHandler};

enum Event {
    Exit,
    LogData(LogDataHolder),
}

pub(super) struct BackgroundWorker {
    join_handle: Option<JoinHandle<()>>,
    tx: Sender<Event>,
}

fn handle_log(
    is_running: &mut bool,
    ev: Event,
    handlers: &mut Vec<Box<dyn StructuralLogHandler>>)
{
    match ev {
        Event::LogData(log_data) => {
            if !log_data.is_empty() {
                for handler in handlers {
                    handler.handle(&log_data);
                }
            }
        },
        Event::Exit => {
            *is_running = false;
        },
    }
}

fn run_in_background(
    rx: &Receiver<Event>,
    mut handlers: Vec<Box<dyn StructuralLogHandler>>)
{
    let mut is_running = true;

    while is_running {
        let ev = rx.recv().expect("BackgroundWorker - rx.recv() fail");
        handle_log(&mut is_running, ev, &mut handlers);

        loop {
            match rx.try_recv() {
                Ok(ev) => handle_log(&mut is_running, ev, &mut handlers),
                Err(err) => {
                    if err == TryRecvError::Empty {
                        break;
                    }
                    panic!("Couldn't handle {err} error on rx.try_recv().");
                }
            }
        }
    }
}

impl BackgroundWorker {
    pub(super) fn new(handlers: Vec<Box<dyn StructuralLogHandler>>) -> Self {
        let (tx, rx) = channel::<Event>();
        let handle = thread::spawn(move || {
            run_in_background(&rx, handlers);
        });

        Self {
            tx: tx,
            join_handle: Some(handle)
        }
    }

    pub(super) fn send_log(&self, log_data: LogDataHolder) {
        self.tx.send(Event::LogData(log_data)).expect("BackgroundWorker - tx.send() fail");
    }
}

impl Drop for BackgroundWorker {
    fn drop(&mut self) {
        if let Some(handle) = self.join_handle.take() {
            self.tx.send(Event::Exit).expect("BackgroundWorker drop - tx.send() fail");
            handle.join().expect("BackgroundWorker drop - handle.join() fail");
        }
    }
}
