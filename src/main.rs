use log::{debug, error, info, log_enabled, Level};
use std::env;
use zmq;

fn main() {
    // set RUST_LOG=debug to see this
    env::set_var("RUST_LOG", "info");

    // initialize the logger
    env_logger::init();

    // create a context
    let context = zmq::Context::new();
    info!("Context created");

    // create a socket
    let socket = context.socket(zmq::REP).unwrap();
    info!("Socket created");

    // bind the socket to a port
    let socket_bind_result = socket.bind("tcp://*:5555");

    match socket_bind_result {
        Ok(_) => {
            info!("Socket bound to port 5555");

            // loop forever
            loop {
                // receive a message
                let msg = socket.recv_string(0).unwrap().unwrap();
                info!("Received message: {}", msg);

                // send a reply with echo
                socket.send(msg.as_bytes(), 0).unwrap();
                info!("Reply sent");
            }
        }
        Err(error) => error!("Socket bind failed: {}", error),
    }
}
