use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll as MioPoll, Token};

use rust_concurrency::runtime::{spawn_task_function, FutureType, Runtime};
use rust_concurrency::spawn_task;
use std::error::Error;
use std::io::{Read, Write};
use std::time::Duration;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::future;

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

struct ServerFuture {
    server: TcpListener,
    poll: MioPoll,
}

impl Future for ServerFuture {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut events = Events::with_capacity(1);

        self.poll
            .poll(&mut events, Some(Duration::from_millis(200)))
            .unwrap();

        for event in events.iter() {
            if event.token() == SERVER && event.is_readable() {
                let (mut stream, _) = self.server.accept().unwrap();
                let mut buf = [0u8; 1024];
                let mut received_data = Vec::new();

                loop {
                    match stream.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            received_data.extend_from_slice(&buf[..n]);
                        }
                        Ok(_) => break,
                        Err(e) => {
                            eprintln!("Error reading from stream: {}", e);
                            break;
                        }
                    }
                }

                if !received_data.is_empty() {
                    let received = String::from_utf8_lossy(&received_data);
                    return Poll::Ready(received.to_string());
                }
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Runtime::new().with_low_num(2).with_high_num(4).run();

    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;
    let mut stream = TcpStream::connect(server.local_addr()?)?;

    let poll: MioPoll = MioPoll::new()?;
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    let server_worker = ServerFuture { server, poll };

    let test = spawn_task!(server_worker);

    let mut client_poll: MioPoll = MioPoll::new()?;
    client_poll
        .registry()
        .register(&mut stream, CLIENT, Interest::WRITABLE)?;

    let mut events = Events::with_capacity(128);
    client_poll.poll(&mut events, None).unwrap();

    for event in events.iter() {
        if event.token() == CLIENT && event.is_writable() {
            let message = "that's so dingo!\n";
            let _ = stream.write_all(message.as_bytes());
        }
    }

    let outcome = future::block_on(test);
    println!("outcome: {}", outcome);

    Ok(())
}
