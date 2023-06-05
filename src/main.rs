use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    //Start
    println!("Initializing the Server!");
    //Creating the listener that will receives connections
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("Successfully, listening in 8080");
    //Create clients connections
    let (sender, _receiver) = broadcast::channel::<(String, SocketAddr)>(10);
    loop {
        //Will await a user connection to continues
        let (mut socket, addr) = listener.accept().await.unwrap();

        //Declarate the sender message and receiver
        let sender = sender.clone();
        let mut receiver = sender.subscribe();

        //Server create a separated task for client
        tokio::spawn(async move {
            //Split the socket into reader and writer
            let (reader, mut writer) = socket.split();

            //After connection ---

            //Creates the buffer storage
            let mut readers = BufReader::new(reader);
            let mut line = String::new();

            //Receives Messages
            loop {
                //Then Function
                tokio::select! {
                    //Read Client message then
                    result = readers.read_line(&mut line) => {
                        //If no message break
                        if result.unwrap() == 0 {
                            break;
                        }
                        //Send the message to other clients
                        sender.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    //Receive the message for other client then
                    result = receiver.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        //If is you the sender then not write for you
                        if addr != other_addr {
                            //Write to the client
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                        line.clear();
                    }
                }
            }
        });
    }
}
