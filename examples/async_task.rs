use async_std::io::prelude::*;
use async_std::net;
use async_std::task;

fn main() -> std::io::Result<()> {
    // let response = task::block_on(cheapo_request("baidu.com", 80, "/"))?;
    // println!("Response: {}", response);
    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
        ("www.baidu.com".to_string(), 80, "/".to_string()),
    ];
    let results = task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("Response: {}", response),
            Err(e) => println!("Error: {}", e),
        }
    }

    let urls = vec![
        "https://example.com".to_string(),
        "https://www.red-bean.com".to_string(),
        "https://en.wikipedia.org".to_string(),
        "https://www.baidu.com".to_string(),
    ];
    let results = task::block_on(many_requests_surf(&urls));
    for result in results {
        match result {
            Ok(response) => println!("Response: {}", response),
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    eprintln!("Requesting {}:{}:{}", host, port, path);
    let mut socket = net::TcpStream::connect((host, port)).await?;
    eprintln!("Connected to {}:{}", host, port);

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    eprintln!("Request write_all {}:{}", host, port);
    socket.shutdown(net::Shutdown::Write)?;
    eprintln!("Request shutdown {}:{}", host, port);

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    eprintln!("Request read_to_string {}:{}", host, port);
    Ok(response)
}

pub async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn(async move {
            cheapo_request(&host, port, &path).await
        }));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

pub async fn many_requests_surf(urls: &[String]) -> Vec<Result<String, surf::Error>> {
    let client = surf::Client::new();

    let mut handles = vec![];
    for url in urls {
        let request = client.get(url).recv_string();
        handles.push(async_std::task::spawn(request));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}
