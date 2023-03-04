use tokio::{net::TcpListener, io::{ AsyncWriteExt,BufReader, AsyncBufReadExt}};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    let (mut socket,_addr) = listener.accept().await.unwrap();


    let (read ,mut write ) = socket.split();

    let mut reader = BufReader::new(read);
    let mut line: String = String::new();

    loop {
        let bytes_read: usize = reader.read_line(&mut line).await.unwrap();

        if bytes_read == 0 {
            break;
        } 
        
        write.write_all(line.as_bytes()).await.unwrap() ;
    }
}   
