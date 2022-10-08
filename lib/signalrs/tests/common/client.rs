use futures::{Sink, SinkExt, Stream};

use flume::{r#async::SendSink, Receiver, Sender};
use signalrs::client::{ChannelSendError, ClientMessage, SignalRClientError};

#[derive(Clone)]
pub struct ClientOutputWrapper<T: 'static> {
    inner: SendSink<'static, T>,
}

impl<T> ClientOutputWrapper<T> {
    pub fn new_text(inner: SendSink<'static, ClientMessage>) -> ClientOutputWrapper<ClientMessage> {
        ClientOutputWrapper { inner: inner }
    }
}

impl Sink<ClientMessage> for ClientOutputWrapper<ClientMessage> {
    type Error = SignalRClientError;

    fn poll_ready(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner
            .poll_ready_unpin(cx)
            .map_err(|x| -> ChannelSendError { x.into() })
            .map_err(|x| -> SignalRClientError { x.into() })
    }

    fn start_send(
        mut self: std::pin::Pin<&mut Self>,
        item: ClientMessage,
    ) -> Result<(), Self::Error> {
        self.inner
            .start_send_unpin(item)
            .map_err(|x| -> ChannelSendError { x.into() })
            .map_err(|x| -> SignalRClientError { x.into() })
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner
            .poll_flush_unpin(cx)
            .map_err(|x| -> ChannelSendError { x.into() })
            .map_err(|x| -> SignalRClientError { x.into() })
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner
            .poll_close_unpin(cx)
            .map_err(|x| -> ChannelSendError { x.into() })
            .map_err(|x| -> SignalRClientError { x.into() })
    }
}