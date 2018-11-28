extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate tokio;
extern crate tokio_serde_bincode;

use tokio::codec::length_delimited;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_serde_bincode::ReadBincode;

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    field: i32,
}

pub fn main() -> Result<(), Box<std::error::Error>> {
    let addr = "127.0.0.1:17653".parse().unwrap();

    // Bind a server socket
    let listener = TcpListener::bind(&addr)?;
    println!("listening on {:?}", listener.local_addr());

    let server = listener
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // Split length delimited frames
            let delimited_stream = length_delimited::Builder::new()
                .new_read(socket)
                .from_err::<bincode::Error>();;
            // Deserialize each frame
            let deserialized: ReadBincode<_, Data> = ReadBincode::new(delimited_stream);
            tokio::spawn(
                deserialized
                    .for_each(|msg| Ok(println!("Got: {:?}", msg)))
                    .map_err(|_| ()),
            );
            Ok(())
        });

    Ok(tokio::run(server))
}
