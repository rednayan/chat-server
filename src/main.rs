use tokio::{net::TcpListener, io::{ AsyncWriteExt,BufReader, AsyncBufReadExt}, sync::broadcast};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    let (tx, rx) = broadcast::channel(10);

    loop {
        let (mut socket,addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {           
                let (read ,mut write ) = socket.split();
            
                let mut reader = BufReader::new(read);
                let mut line: String = String::new();

                loop {
                    tokio::select! {
                        result = reader.read_line(&mut line) => {
                            if result.unwrap() == 0 {
                                break;
                            }
                            tx.send((line.clone(),addr)).unwrap();
                            line.clear();
                        }
                        result= rx.recv() => {
                            let (msg,other_addr) = result.unwrap();
                            
                            if addr != other_addr {
                                write.write_all(msg.as_bytes()).await.unwrap();
                            }
                        }
                    }
                }  
        });
    }
}   
