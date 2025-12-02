extern crate tinyrick_models;

use tinyrick_macros::{default_task, task};

use std::sync;

static X: sync::LazyLock<sync::Mutex<u64>> = sync::LazyLock::new(|| sync::Mutex::new(0));

static Z: sync::LazyLock<sync::Mutex<u64>> = sync::LazyLock::new(|| sync::Mutex::new(0));

#[default_task]
fn increment_x() {
    let mut y = X.lock().unwrap();
    *y += 1;
}

#[test]
fn test_default_task() {
    assert!(*X.lock().unwrap() == 0);
    let t_guard = tinyrick_models::DEFAULT_TASK.lock().unwrap();
    let t = t_guard.as_ref().unwrap_or(&*tinyrick_models::NOP_BOX);
    t();
    assert!(*X.lock().unwrap() == 1);
}

#[task]
fn increment_z() {
    let mut w = Z.lock().unwrap();
    *w += 1;
}

#[test]
fn test_task() {
    assert!(*Z.lock().unwrap() == 0);
    let t = &tinyrick_models::TASKS.lock().unwrap()["increment_z"];
    t();
    assert!(*Z.lock().unwrap() == 1);
}
