use prom_server::Server;

fn main() {
    let server = Server::default();

    server.listen("127.0.0.1:7777");
}

#[cfg(test)]
mod tests {
    use prom_server::Client;

    #[test]
    fn test_client() {
        if let Ok(client) = Client::connect("127.0.0.1:7777") {
            println!("{:#?}", client.get_projects());
        }
    }
}
