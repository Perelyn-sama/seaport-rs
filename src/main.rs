mod constants;
mod contracts;
mod playground;
mod seaport;
mod types;

// TODO The only use I have for this file is to run Playground, I might remove this later
fn main() {
    playground::run()
}
