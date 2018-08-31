extern crate mio;
use mio::tcp::*;
use mio::*;
use std::net::SocketAddr;

struct WebSocketServer;

impl Handler for WebSocketServer {
    // Traits can have useful default implementations, so in fact the handler
    // interface requires us to provide only two things: concrete types for
    // timeouts and messages.
    // We're not ready to cover these fancy details, and we wouldn't get to them
    // anytime soon, so let's get along with the defaults from the mio examples:
    type Timeout = usize;
    type Message = ();
}

fn main() {
    // Open and bind a TCP port:
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    // Register socket with event loop:
    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&server_socket,
                        Token(0),
                        EventSet::readable(),
                        PollOpt::edge()).unwrap();

    // Create a new instance of our handler struct:
    let mut handler = WebSocketServer;
    // ... and then provide the event loop with a mutable reference to it:
    event_loop.run(&mut handler).unwrap();
}
