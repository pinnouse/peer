mod decode_torrent;
mod net_client;
mod udp_client;

use bendy::decoding::{Error, FromBencode};
use std::fs;

pub fn parse_bencode(file_contents: &[u8]) -> Result<decode_torrent::MetaInfo, Error> {
    decode_torrent::MetaInfo::from_bencode(file_contents)
}

pub fn read_torrent_file(filename: &str) -> Result<decode_torrent::MetaInfo, Error> {
    let file_contents = fs::read(filename)?;
    parse_bencode(file_contents.as_slice())
}
