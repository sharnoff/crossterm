//! Demonstrates how to read events asynchronously with tokio.
//!
//! cargo run --features event-stream --example event-stream-tokio

use std::{io::{stdout, Write}, time::{Instant, Duration}, thread};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use futures::stream::Next;
use futures::future::{Fuse, JoinAll, join};
use std::thread::spawn;
use futures::stream::TryStreamExt;
use std::iter::FromIterator;
use tokio::runtime::Runtime;
use std::sync::{Mutex, Arc};
use futures::executor::block_on;
use tokio::task;

async fn print_events(name: &str, duration: Duration) {
    let mut reader = EventStream::new();

    let instant = Instant::now();

    loop {
        if instant.elapsed() > duration {
            break;
        }

        let mut event = reader.next().fuse();
        let mut delay = Delay::new(duration).fuse();

        select! {
            _ = delay => { drop(reader); break; },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("ffrom {} | Event::{:?} \r ", name, event);
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;

    let thread_a = thread::spawn(move|| {
        println!("EventStream A");
        block_on(print_events("Thread A", Duration::from_secs(5)));
    });

    let thread_b = thread::spawn(move || {
        println!("EventStream B");
        block_on(print_events("Thread B", Duration::from_secs(8)));
    });

    println!("joining A"); /* stream a is dropped after 5 secs */
    thread_a.join();
    println!("joining B"); /* stream b is dropped after 8 secs */
    thread_b.join();

    println!("joined A and B");

    disable_raw_mode()
}
