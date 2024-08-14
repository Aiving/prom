use std::io::Read;

use serde::{de::DeserializeOwned, Serialize};

pub struct Packet {
    pub data: Vec<u8>,
    pub size: [u8; 8],
}

impl Packet {
    pub fn new<T: Serialize>(data: &T) -> Self {
        let data = bson::to_vec(data).expect("failed to serialize packet data");
        let size = data.len().to_be_bytes();

        Self { data, size }
    }

    fn receive_length<R: Read>(buffer: &mut R) -> Option<usize> {
        let mut length = [0; 8];

        match buffer.read(&mut length) {
            Ok(size) => {
                if size == 0 {
                    return None;
                }
            }
            Err(err) => {
                println!("failed to read packet length: {err}");

                return None;
            }
        }

        Some(usize::from_be_bytes(length))
    }

    pub fn receive<R: Read, T: DeserializeOwned>(buffer: &mut R) -> Option<T> {
        let length = Self::receive_length(buffer)?;

        let mut message = vec![0; length];

        if buffer.read_exact(&mut message).is_err() {
            println!("failed to read message");

            return None;
        }

        bson::from_slice::<T>(&message).ok()
    }
}
