mod app;
mod start;
mod service;
mod pdf;
mod network;
mod crypt;

fn main() {
    app::run().expect("TODO: Error message");
}