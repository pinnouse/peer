use std::net::UdpSocket;
use std::io::{Error, ErrorKind, Result, Cursor, Read, Write};
use super::net_client::NetClient;
use rand::{thread_rng, Rng};
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use core::num::FpCategory::Infinite;
use log::{info, warn};

const CLIENT_ID: &str = "PP";
const CLIENT_VERSION: &str = "0001";

const MAX_BYTES: usize = 16384;
const MAGIC_CONSTANT: i64 = 0x41727101980;

const INVALID_RESPONSE_ERR: Error = Error::new(ErrorKind::InvalidData, "Invalid response received.");
const INVALID_TRANSACTION_ID: Error = Error::new(ErrorKind::InvalidData, "Invalid transaction ID, source not trusted.");
const INVALID_CONNECTION_ID: Error = Error::new(ErrorKind::InvalidData, "Invalid connection ID, source not trusted.");

pub struct UdpClient<'a> {
    socket: &'a mut UdpSocket,
    peer_id: &'a [u8],
    transaction_id: Option<i32>,
    connection_id: Option<i64>,
}

impl NetClient for UdpClient {
    fn request(&self, url: &str, data: &[u8]) -> Result<&[u8]> {
        self.socket.connect(url)?;
        self.socket.send(data);
        let mut buf = [0; MAX_BYTES];
        let received = self.socket.recv(&mut buf)?;
        Ok(&buf[..received])
    }
}

impl UdpClient {
    pub fn new() -> Result<UdpClient> {
        let addrs = [
            SocketAddr::from(([127, 0, 0, 1], 32222)),
            SocketAddr::from(([127, 0, 0, 1], 34444)),
            SocketAddr::from(([127, 0, 0, 1], 34455)),
        ];
        let mut socket = UdpSocket::bind(&addrs[..])?;
        let peer_gen = (0..12)
            .map(|_| {
                let x: u8 = thread_rng().gen_range(0, 10);
                x.to_string()
            })
            .collect();
        let peer_id = format!("-{}{}-{:012}", CLIENT_ID, CLIENT_VERSION, peer_gen);
        let peer_id_bytes = peer_id.as_bytes();
        if peer_id_bytes.len() != 20 {
            warn!("Peer ID created incorrectly: {}", peer_id)
        }
        UdpClient(socket, peer_id_bytes)
    }

    fn create_transaction_id(&self) -> i32 {
        let transaction_id = thread_rng().gen::<i32>();
        *self.transaction_id = transaction_id;
        transaction_id
    }

    fn get_transaction_id(&self) -> i32 {
        if let Some(x) = *self.transaction_id {
            x
        }
        return 0
    }

    fn handle_err_response(&self, response: &[u8]) {
        if response.len() < 8 {
            warn!("Invalid response, expected len greater than 8, got len: {}", response.len())
            return
        }
        let mut rdr = Cursor::new(response);
        rdr.set_position(3);
        let transaction_id = rdr.read_i32::<BigEndian>();
        if transaction_id != self.get_transaction_id() {
            warn!("Invalid transaction ID received, expected {}, got {}", self.get_transaction_id(), response_transaction_id);
        }
        let mut message: String;
        rdr.read_to_string(&mut message);
        warn!(target: "response_errors", message)
    }

    fn build_connect_request(&self) -> Result<&[u8]> {
        let mut conn_req = vec![];
        conn_req.write_i64::<BigEndian>(MAGIC_CONSTANT)?;
        conn_req.write_i32::<BigEndian>(0)?;
        conn_req.write_i32::<BigEndian>(self.create_transaction_id());
        Ok(conn_req.as_ref())
    }

    pub fn connect_request(&self) -> Result<()> {
        let conn_req = self.build_connect_request()?;
        let response = self.request(url, conn_req)?;
        let mut rdr = Cursor::new(response);
        let response_action = rdr.read_i32::<BigEndian>()?;
        if response_action == 3 {
            self.handle_err_response(response);
            Err(INVALID_RESPONSE_ERR)
        }

        if response.len() != 16 {
            warn!("Invalid length of response, expected 16, got: {}.", response.len());
            Err(INVALID_RESPONSE_ERR)
        }

        if response_action != 0 {
            warn!("Invalid action response, expected 0, got: {}.", response_action);
            Err(INVALID_RESPONSE_ERR)
        }
        let response_transaction_id = rdr.read_i32::<BigEndian>()?;
        if response_transaction_id != self.get_transaction_id() {
            warn!("Invalid transaction ID received, expected {}, got {}", self.get_transaction_id(), response_transaction_id);
            Err(INVALID_TRANSACTION_ID)
        }
        let response_connection_id = rdr.read_i64::<BigEndian>()?;
        *self.connection_id = response_connection_id;
        Ok(())
    }

    fn build_announce_request(&self, info_hash: &[u8]) -> Result<&[u8]> {
        let mut announce_req = vec![];
        let Some(connection_id) = *self.connection_id;
        if *self.connection_id == None {
            warn!("Connection ID is not set, make a connection request first.");
            Err(INVALID_CONNECTION_ID)
        }
        announce_req.write_i64::<BigEndian>(connection_id)?;
        announce_req.write_i32::<BigEndian>(1)?;
        announce_req.write_i32::<BigEndian>(self.create_transaction_id())?;
        announce_req.write(info_hash);
        announce_req.write(self.peer_id);

        Ok(announce_req.as_ref())
    }

    pub fn announce_request(&self) -> Result<()> {
        Ok(())
    }
}