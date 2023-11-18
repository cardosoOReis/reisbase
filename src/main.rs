use terminal_communication::TerminalCommunication;

pub mod reisbase;
pub mod controller;
pub mod failures;
pub mod error_handler;
pub mod constants;
pub mod interface;
pub mod sucess;
pub mod terminal_communication;
pub mod arguments;
pub mod extensions;

fn main() {
    TerminalCommunication::execute()
}
