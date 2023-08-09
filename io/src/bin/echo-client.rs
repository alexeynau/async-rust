use mini_redis::Result;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    // init data
    let data = b"Hello, server!";
    // send data to stream
    stream.try_write(data).unwrap();

    let mut buf = vec![0; 1024];
    // get response
    stream.peek(&mut buf).await.unwrap();
    // transform response to stream
    let response = String::from_utf8_lossy(&buf[..])
        .trim_end_matches('\0')
        .to_string();
    println!("{:?}", response);
    Ok(())
}
