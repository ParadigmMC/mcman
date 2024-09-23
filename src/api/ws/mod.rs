//! WebSocket Server implementation for third party editors

use std::sync::Arc;

use anyhow::{Context, Result};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use msg::{MsgIn, MsgOut};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

use super::app::App;

pub mod msg;

pub type WebsocketSink = SplitSink<WebSocketStream<TcpStream>, Message>;
pub type WebsocketStream = SplitStream<WebSocketStream<TcpStream>>;

pub struct WebsocketServer {
    app: Arc<App>,
    clients: RwLock<Vec<WebsocketSink>>,
}

impl WebsocketServer {
    pub fn new(app: Arc<App>) -> Arc<Self> {
        Arc::new(Self {
            app,
            clients: RwLock::new(vec![]),
        })
    }

    pub async fn start(self: Arc<Self>, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr)
            .await
            .with_context(|| format!("Listening to addr: {addr}"))?;

        while let Ok((stream, addr)) = listener.accept().await {
            println!("New connection: {addr:?}");
            tokio::spawn(self.clone().handle_connection(stream));
        }

        Ok(())
    }

    pub async fn handle_connection(self: Arc<Self>, stream: TcpStream) -> Result<()> {
        let (sink, mut stream) = accept_async(stream)
            .await
            .expect("Failed to accept")
            .split();

        {
            let mut clients = self.clients.write().await;

            clients.push(sink);
        }

        while let Some(Ok(msg)) = stream.next().await {
            let text = msg.into_text()?;
            let event: MsgIn = serde_json::from_str(&text)?;
            self.clone().handle_event(event).await?;
        }

        Ok(())
    }

    pub async fn broadcast(self: Arc<Self>, event: MsgOut) -> Result<()> {
        let mut clients = self.clients.write().await;

        let msg: Message = event.into();

        let mut to_remove = vec![];

        for (i, client) in clients.iter_mut().enumerate() {
            if let Err(_) = client.send(msg.clone()).await {
                to_remove.push(i);
            };
        }

        for i in to_remove {
            let _ = clients.remove(i);
        }

        Ok(())
    }

    pub async fn handle_event(self: Arc<Self>, event: MsgIn) -> Result<()> {
        Ok(())
    }
}
