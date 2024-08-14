use std::{
    fs::{self, File},
    io::{self, Write},
    net::TcpStream,
    path::PathBuf,
};

use directories::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};

use crate::packet::Packet;

pub static ROOT: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("kz", "Aiving", "Prom")
        .unwrap()
        .config_dir()
        .to_path_buf()
});

pub fn try_read_file<T: Default + Serialize + DeserializeOwned>(path: PathBuf) -> io::Result<T> {
    if !path.exists() {
        fs::write(
            &path,
            serde_json::to_vec(&T::default()).expect("failed to serialize data"),
        )?;
    }

    File::open(path).map(|file| serde_json::from_reader(file).expect("failed to deserialize file"))
}

pub trait StreamExt {
    fn send_packet(&mut self, packet: Packet) -> io::Result<()>;
}

impl StreamExt for TcpStream {
    fn send_packet(&mut self, packet: Packet) -> io::Result<()> {
        self.write_all(&packet.size)?;
        self.write_all(&packet.data)?;

        self.flush()
    }
}
