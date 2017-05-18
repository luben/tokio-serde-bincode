//! `Stream` and `Sink` adaptors for serializing and deserializing values using
//! Bincode.
//!
//! This crate provides adaptors for going from a stream or sink of buffers
//! ([`Bytes`]) to a stream or sink of values by performing Bincode encoding or
//! decoding. It is expected that each yielded buffer contains a single
//! serialized Bincode value. The specific strategy by which this is done is left
//! up to the user. One option is to use using [`length_delimited`] from
//! [tokio-io].
//!
//! [`Bytes`]: https://docs.rs/bytes/0.4/bytes/struct.Bytes.html
//! [`length_delimited`]: http://alexcrichton.com/tokio-io/tokio_io/codec/length_delimited/index.html
//! [tokio-io]: http://github.com/alexcrichton/tokio-io
//! [examples]: https://github.com/carllerche/tokio-serde-json/tree/master/examples

extern crate futures;
extern crate bytes;
extern crate serde;
extern crate bincode;
extern crate tokio_serde;

use futures::{Stream, Poll, Sink, StartSend};
use bytes::{Bytes, BytesMut};
use serde::{Serialize, Deserialize};
use bincode::Error;
use tokio_serde::{Serializer, Deserializer, FramedRead, FramedWrite};

use std::io;
use std::marker::PhantomData;

/// Adapts a stream of Bincode encoded buffers to a stream of values by
/// deserializing them.
///
/// `ReadBincode` implements `Stream` by polling the inner buffer stream and
/// deserializing the buffer as Bincode. It expects that each yielded buffer
/// represents a single Bincode value and does not contain any extra trailing
/// bytes.
pub struct ReadBincode<T, U> {
    inner: FramedRead<T, U, Bincode<U>>,
}

/// Adapts a buffer sink to a value sink by serializing the values as Bincode.
///
/// `WriteBincode` implements `Sink` by serializing the submitted values to a
/// buffer. The buffer is then sent to the inner stream, which is responsible
/// for handling framing on the wire.
pub struct WriteBincode<T: Sink, U> {
    inner: FramedWrite<T, U, Bincode<U>>,
}

struct Bincode<T> {
    ghost: PhantomData<T>,
}

impl<T, U> ReadBincode<T, U>
    where T: Stream<Error = io::Error>,
          U: for<'de> Deserialize<'de>,
          Bytes: From<T::Item>,
{
    /// Creates a new `ReadBincode` with the given buffer stream.
    pub fn new(inner: T) -> ReadBincode<T, U> {
        let json = Bincode { ghost: PhantomData };
        ReadBincode { inner: FramedRead::new(inner, json) }
    }
}

impl<T, U> ReadBincode<T, U> {
    /// Returns a reference to the underlying stream wrapped by `ReadBincode`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    /// Returns a mutable reference to the underlying stream wrapped by
    /// `ReadBincode`.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise
    /// being worked with.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Consumes the `ReadBincode`, returning its underlying stream.
    ///
    /// Note that care should be taken to not tamper with the underlying stream
    /// of data coming in as it may corrupt the stream of frames otherwise being
    /// worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T, U> Stream for ReadBincode<T, U>
    where T: Stream<Error = io::Error>,
          U: for<'de> Deserialize<'de>,
          Bytes: From<T::Item>,
{
    type Item = U;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<U>, Error> {
        self.inner.poll()
    }
}

impl<T, U> Sink for ReadBincode<T, U>
    where T: Sink,
{
    type SinkItem = T::SinkItem;
    type SinkError = T::SinkError;

    fn start_send(&mut self, item: T::SinkItem)
                  -> StartSend<T::SinkItem, T::SinkError> {
        self.get_mut().start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), T::SinkError> {
        self.get_mut().poll_complete()
    }

    fn close(&mut self) -> Poll<(), T::SinkError> {
        self.get_mut().close()
    }
}

impl<T, U> WriteBincode<T, U>
    where T: Sink<SinkItem = BytesMut, SinkError = io::Error>,
          U: Serialize,
{
    /// Creates a new `WriteBincode` with the given buffer sink.
    pub fn new(inner: T) -> WriteBincode<T, U> {
        let json = Bincode { ghost: PhantomData };
        WriteBincode { inner: FramedWrite::new(inner, json) }
    }
}

impl<T: Sink, U> WriteBincode<T, U> {
    /// Returns a reference to the underlying sink wrapped by `WriteBincode`.
    ///
    /// Note that care should be taken to not tamper with the underlying sink as
    /// it may corrupt the sequence of frames otherwise being worked with.
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    /// Returns a mutable reference to the underlying sink wrapped by
    /// `WriteBincode`.
    ///
    /// Note that care should be taken to not tamper with the underlying sink as
    /// it may corrupt the sequence of frames otherwise being worked with.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Consumes the `WriteBincode`, returning its underlying sink.
    ///
    /// Note that care should be taken to not tamper with the underlying sink as
    /// it may corrupt the sequence of frames otherwise being worked with.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T, U> Sink for WriteBincode<T, U>
    where T: Sink<SinkItem = BytesMut, SinkError = io::Error>,
          U: Serialize,
{
    type SinkItem = U;
    type SinkError = io::Error;

    fn start_send(&mut self, item: U) -> StartSend<U, io::Error> {
        self.inner.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), io::Error> {
        self.inner.poll_complete()
    }

    fn close(&mut self) -> Poll<(), io::Error> {
        self.inner.poll_complete()
    }
}

impl<T, U> Stream for WriteBincode<T, U>
    where T: Stream + Sink,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<T::Item>, T::Error> {
        self.get_mut().poll()
    }
}

impl<T> Deserializer<T> for Bincode<T>
    where T: for<'de> Deserialize<'de>
{
    type Error = Error;

    fn deserialize(&mut self, src: &Bytes) -> Result<T, Error> {
        bincode::deserialize(src)
    }
}

impl<T: Serialize> Serializer<T> for Bincode<T> {
    type Error = io::Error;

    fn serialize(&mut self, item: &T) -> Result<BytesMut, io::Error> {
        bincode::serialize(item, bincode::Infinite)
            .map(Into::into)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
