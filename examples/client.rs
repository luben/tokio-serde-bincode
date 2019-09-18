extern crate bincode;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate tokio;
extern crate tokio_serde_bincode;

use futures::sink;
use tokio::codec::{FramedWrite, LengthDelimitedCodec, length_delimited};
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

// FramedWrite upgrades TcpStream from an AsyncWrite to a Sink
type IOErrorSink = FramedWrite<TcpStream, LengthDelimitedCodec>;

// sink::SinkFromErr maps underlying IO errors into Bincode errors
type BincodeErrSink = sink::SinkFromErr<IOErrorSink, bincode::Error>;

// WriteBincode maps Bincode-serializeable structs into bytes
type BincodeSink = WriteBincode<BincodeErrSink, Data>;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:17653".parse()?;

    // Bind a server socket
    let client = TcpStream::connect(&addr)
        .from_err()
        .and_then(|socket| {
            // Delimit frames using length prefix
            let delimited_sink: BincodeErrSink = length_delimited::Builder::new()
                .new_write(socket)
                .sink_from_err::<bincode::Error>();
            // Serialize frames
            let serialized: BincodeSink = WriteBincode::new(delimited_sink);
            serialized.send(Data { field: 42 }).map(|_result| ())
        })
        .map_err(|err| println!("Error {:?}", err));
    Ok(tokio::run(client))
}
