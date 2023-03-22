use std::net::TcpListener;

use net::listener::ClientConnectionListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let login_task = tokio::spawn(login());
    let world_task = tokio::spawn(world());

    tokio::select! {
      _ = login_task => {
        println!("SIGINT received, shutting down");
      }
      _ = world_task => {
        println!("SIGINT received, shutting down");
      }
    }
    Ok(())
}

async fn login() {
    let listener = TcpListener::bind("0.0.0.0:8484").unwrap();
    println!("Listening on 8484");
    loop {
        let (socket, _) = listener.accept().unwrap();
        println!(
            "Connection Terminated: {}",
            ClientConnectionListener::login_server(socket)
                .and_then(|mut session| session.listen())
                .expect_err("Thread disconnects should result in error...")
        )
    }
}

async fn world() {
    let listener = TcpListener::bind("0.0.0.0:8485").unwrap();
    println!("Listening on 8485");
    loop {
        let (socket, _) = listener.accept().unwrap();
        println!(
            "Connection Terminated: {}",
            ClientConnectionListener::world_server(socket)
                .and_then(|mut session| session.listen())
                .expect_err("Thread disconnects should result in error...")
        )
    }
}
