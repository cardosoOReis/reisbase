use terminal_communication::TerminalCommunication;

pub mod actions;
pub mod arguments;
pub mod constants;
pub mod controller;
pub mod error_handler;
pub mod extensions;
pub mod failures;
pub mod interface;
pub mod reisbase;
pub mod sucess;
pub mod terminal_communication;

fn main() {
    TerminalCommunication::execute()
}
