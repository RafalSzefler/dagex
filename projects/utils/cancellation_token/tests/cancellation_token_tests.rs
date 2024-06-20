use std::{sync::{Arc, Mutex}, thread, time::Duration};

use cancellation_token::CancellationTokenSource;

#[test]
fn test_simple_cancellation() {
    let mut cts = CancellationTokenSource::default();
    let value = Arc::new(Mutex::new(5));

    let get_clone = value.clone();
    let get = || {
        let guard = get_clone.lock().unwrap();
        *guard
    };

    let mut token = cts.token();
    let clone = value.clone();
    let _ = token.register(move || {
        let mut _guard = clone.lock().unwrap();
        *_guard = 15;
    });
    assert_eq!(get(), 5);
    cts.cancel().unwrap();
    assert_eq!(get(), 15);
}


#[test]
fn test_threaded_cancellation() {
    const INITIAL: i32 = 5;
    const POTENTIAL: i32 = 15;
    const THREAD_COUNT: usize = 20;
    let value = Arc::new(Mutex::new(INITIAL));

    let get_clone = value.clone();
    let get = || {
        let guard = get_clone.lock().unwrap();
        *guard
    };

    let mut cts = CancellationTokenSource::default();
    let token = cts.token();
    assert!(token.is_cancelled().is_ok());
    assert!(cts.cancel().is_ok());
    assert!(cts.cancel().is_err());

    let mut handles = Vec::with_capacity(THREAD_COUNT);
    for _ in 0..THREAD_COUNT {
        let token = cts.token();
        let set_clone = value.clone();
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            if token.is_cancelled().is_err() {
                return;
            }
            let mut guard = set_clone.lock().unwrap();
            *guard = POTENTIAL;
        }));
    }

    assert_eq!(get(), INITIAL);
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
    assert_eq!(get(), INITIAL);
}

#[test]
fn test_threaded_cancellation_2() {
    let mut cts = CancellationTokenSource::default();
    let token = cts.token();
    assert!(token.is_cancelled().is_ok());

    let counter = Arc::new(Mutex::new(0usize));

    let get_clone = counter.clone();
    let get = || {
        let guard = get_clone.lock().unwrap();
        *guard
    };

    assert_eq!(get(), 0);

    let counter_clone = counter.clone();
    let handle = thread::spawn(move || {
        while token.is_cancelled().is_ok() {
            let mut guard = counter_clone.lock().unwrap();
            *guard = *guard + 1;
        }
    });

    thread::sleep(Duration::from_millis(100));
    let last_value = get();
    assert!(last_value > 10);
    assert!(!handle.is_finished());
    assert!(cts.cancel().is_ok());
    handle.join().unwrap();
    assert!(get() >= last_value);
}
