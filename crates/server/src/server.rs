use std::{
    io::BufReader,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::{Arc, RwLock},
    thread,
};

use prom_data::{Request, Response};

use crate::{app::App, packet::Packet, util::StreamExt};

#[derive(Default, Clone)]
pub struct Server {
    app: Arc<RwLock<App>>,
}

impl Server {
    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = BufReader::new(stream.try_clone().unwrap());

        while let Some(data) = Packet::receive::<_, Request>(&mut buffer) {
            match data {
                Request::GetProjects => {
                    let app = self.app.read().expect("failed to borrow app");

                    stream
                        .send_packet(Packet::new(&Response::GetProjects(app.projects.clone())))
                        .unwrap();
                }
                Request::AddProject(project) => {
                    let mut app = self.app.write().expect("failed to borrow app as mutable");

                    app.projects.push(project);

                    app.save_projects().expect("failed to save projects");
                }
                Request::OpenProject(id) => println!("open project at {id}"),
                Request::SetEditor(editor) => {
                    let mut app = self.app.write().expect("failed to borrow app as mutable");

                    app.config.editor = editor;

                    app.save_config().expect("failed to save config");
                }
            }
        }
    }

    pub fn listen<A: ToSocketAddrs>(&self, addr: A) {
        let listener = TcpListener::bind(addr).unwrap();
        let mut stream = listener.incoming();

        while let Some(Ok(stream)) = stream.next() {
            let server = self.clone();

            thread::spawn(move || server.handle_connection(stream));
        }
    }
}
