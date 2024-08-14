use crate::{packet::Packet, util::StreamExt};
use prom_data::{Editor, Project, Request, Response};
use std::{
    io,
    net::{TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
};

pub struct Client {
    stream: Arc<Mutex<TcpStream>>,
}

impl Client {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;

        Ok(Self {
            stream: Arc::new(Mutex::new(stream)),
        })
    }

    fn send_request(&self, request: Request) -> io::Result<()> {
        let mut stream = self.stream.lock().expect("failed to lock stream");

        stream.send_packet(Packet::new(&request))
    }

    fn get_response(&self) -> Option<Response> {
        let mut stream = self.stream.lock().expect("failed to lock stream");

        Packet::receive(&mut *stream)
    }

    pub fn get_projects(&self) -> Vec<Project> {
        self.send_request(Request::GetProjects)
            .expect("failed to send request");

        self.get_response()
            .map(|response| match response {
                Response::GetProjects(projects) => projects,
            })
            .unwrap_or_default()
    }

    pub fn set_editor(&self, editor: Editor) {
        self.send_request(Request::SetEditor(editor))
            .expect("failed to send request");
    }

    pub fn add_project(&self, project: Project) {
        self.send_request(Request::AddProject(project))
            .expect("failed to send request");
    }
}
