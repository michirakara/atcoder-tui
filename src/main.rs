mod backend;
mod handler;
mod run;
mod ui;

// TODO: use crossterm::event::EventStream to make it async

fn main() {
    run::run();
}
