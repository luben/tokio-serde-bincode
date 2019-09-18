extern crate bincode;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate tokio;
extern crate tokio_serde_bincode;

use futures::stream;
use tokio::codec::{FramedRead, LengthDelimitedCodec, length_delimited};
use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;
use tokio_serde_bincode::ReadBincode;

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    #[cfg(i128)]
    field: i128,
    #[cfg(not(i128))]
    field: i32,
}

// FramedRead upgrades TcpStream from an AsyncRead to a Stream
type IOErrorStream = FramedRead<TcpStream, LengthDelimitedCodec>;

// stream::FromErr maps underlying IO errors into Bincode errors
type BincodeErrStream = stream::FromErr<IOErrorStream, bincode::Error>;

// ReadBincode maps underlying bytes into Bincode-deserializable structs
type BincodeStream = ReadBincode<BincodeErrStream, Data>;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:17653".parse().unwrap();

    // Bind a server socket
    let listener = TcpListener::bind(&addr)?;
    println!("listening on {:?}", listener.local_addr());

    let server = listener
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // Split length delimited frames
            let delimited_stream: BincodeErrStream = length_delimited::Builder::new()
                .new_read(socket)
                .from_err::<bincode::Error>();;
            // Deserialize each frame
            let deserialized: BincodeStream = ReadBincode::new(delimited_stream);
            tokio::spawn(
                deserialized
                    .for_each(|msg| Ok(println!("Got: {:?}", msg)))
                    .map_err(|_| ()),
            );
            Ok(())
        });

    Ok(tokio::run(server))
}
