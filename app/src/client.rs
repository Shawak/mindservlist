use async_std::net::UdpSocket;
use async_std::io;
use std::io::Error;
use std::time::{Duration, Instant};

use crate::now;
use crate::server::*;

pub struct Client {
    socket: UdpSocket
}

impl Client {
    pub async fn new() -> Result<Client, Error> {
        Ok(Client {
            socket: UdpSocket::bind("0.0.0.0:0").await?
        })
    }
    
    pub async fn get(&mut self, address: &String) -> Result<Server, Error> {
        // https://github.com/Anuken/Mindustry/blob/1fffbf3a790ff0e904eed675afa426748cedf9f2/core/src/mindustry/net/ArcNetProvider.java#L384
        static REQUEST: &'static [u8] = &[-2i8 as _, 1u8];
        self.socket.send_to(REQUEST, address).await?;
        let then = Instant::now();

        let mut buffer = vec![0u8; 512];
        let (_n, peer) = io::timeout(Duration::from_secs(3), async {
            self.socket.recv_from(&mut buffer).await
        }).await?;

        let ping = then.elapsed().as_millis();
        let data: &mut &[u8] = &mut buffer.as_ref();
        let server = Server::new(peer.to_string(), address.clone(), now(), ping as _, data)?;

        Ok(server)
    }
}