extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate tokio;
extern crate tokio_serde_bincode;

use tokio::codec::length_delimited;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio_serde_bincode::WriteBincode;

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    #[cfg(i128)]
    field: i128,
    #[cfg(not(i128))]
    field: i32,
}

pub fn main() -> Result<(), Box<std::error::Error>> {
    let addr = "127.0.0.1:17653".parse()?;

    // Bind a server socket
    let client = TcpStream::connect(&addr)
        .from_err()
        .and_then(|socket| {
            // Delimit frames using length prefix
            let delimited_sink = length_delimited::Builder::new()
                .new_write(socket)
                .sink_from_err::<bincode::Error>();

            // Serialize frames
            let serialized: WriteBincode<_, Data> = WriteBincode::new(delimited_sink);
            serialized.send(Data { field: 42 }).map(|_result| ())
        })
        .map_err(|err| println!("Error {:?}", err));
    Ok(tokio::run(client))
}
