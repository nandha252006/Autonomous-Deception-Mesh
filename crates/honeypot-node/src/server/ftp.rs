use tokio::net::{TcpLISTener,TcpStream};
use tokio::io::{AsyncBufReadExt,BufReader,AsyncWriteExt};
struct Session{
    username: Option<String>,
    authenticated: bool,
    cwd: String,
    passive_listener:Option<String>,
    ipadd: String,
}
struct Logevent{
    protocol:String,
    time: String,
    ip: String,
    command: String,
    data: String,
}
enum Command{
    USER(String),
    PASS(String),
    LIST,
    RETR(String),
    Stor(String),
    PASS,
    QUIT,
    Unknown,
}
fn parse_command(line:&str)->Command {
    let part:Vec<&str> = line.split_whitespace().collect();
    match part.get(0){
        Some(&"USER")=>Command::User(part.get(1).unwrap_or(&"").to_string()),
        Some(&"PASS")=>Command::Pass(part.get(1).unwrap_or(&"").to_string()),
        Some(&"LIST")=>Command::List,
        Some(&"RETR")=>Command::Retr(part.get(1).unwrap_or(&"").to_string()),
        Some(&"STOR")=>Command::Stor(part.get(1).unwrap_or(&"").to_string()),
        Some(&"PASS")=>Command::Pasv,
        Some(&"QUIT")=>Command::Quit,
        _ => Command::Unknown,
    }
}
async fn list_file()-> String{
    "drwxr-xr-x 1 root root 4096 etc\r\n-rw-r--r-- 1 root root 123 passwd\r\n".to_string()
}
async fn logevent(log:Logevent){
    println!("{} {} {} {} {}",log.protocol,log.time,log.ip,log.command,log.data);
}
async fn handle_client(stream:TcpStream,addr:std::net::SocketAddr){
    let (reader, mut writter)=stream.into_split();
    let mut reader=BufReader::new(reader);
    let mut session= Session{
        username:None,
        authenticated:false,
        cwd:"/".to_string(),
        passive_listener:None,
        ipadd:addr.to_string(),
    };
    let mut line=String::new();
    let _=writter.write_all(b"Welcome ftp server");
    loop{
        line.clear();
        if reader.read_line(&mut line).await.unwrap() == 0 {
            break;
        }
        let cmd=parse_command(line.trim());
        match cmd{
            Command::USER(u)=> {
                session.username=Some(u.clone());
                let msg=format!("Registered the username as {}",u);
                writter.write_all(msg.as_bytes()).await.unwrap();
                logevent(Logevent{
                    protocol:"FTP".to_string(),
                    time:chrono::offset::Local::now().to_string(),
                    ip:addr.to_string();
                    command:"USER".to_string(),
                    data:u.clone(),
                });
            },
            Command::PASS(p)=>{
                session.authenticated=true;
                writter.write_all(b"authenticated inside the ftp server\r\n").await.unwrap();
                logevent(Logevent{
                    protocol:"FTP".to_string(),
                    time:chrono::offset::Local::now().to_string(),
                    ip:addr.to_string();
                    command:"PASS".to_string(),
                    data:p.clone(),
                });
            },
            Command::LIST=>{
                let file_system=list_file().await;
                writter.write_all(list_file().await.as_bytes()).await.unwrap();
                logevent(Logevent{
                    protocol:"FTP".to_string(),
                    time:chrono::offset::Local::now().to_string(),
                    ip:addr.to_string();
                    command:"LIST".to_string(),
                    data:"Success".into(),
                });
            }
            Command::QUIT=>{
                writter.write_all(b"Exited the ftp server\r\n").await.unwrap();
                logevent(Logevent{
                    protocol:"FTP".to_string(),
                    time:chrono::offset::Local::now().to_string(),
                    ip:addr.to_string();
                    command:"QUIT".to_string(),
                    data:"Success".into(),
                });
                break;
            }
            _=>{
                let message=format!("Unknown Command");
                let _=writter.write_all(message.as_bytes());
                break;
            }
        }
    }
}
pub async fn start_ftp(){
    let listener=TcpLISTener::bind("0.0.0.0:2121").await.unwrap();
    println!("Executing ftp");
    loop {
        let (stream,addr) = listener.accept().await.unwrap();
        tokio::spawn(handle_client(stream,addr));
    }
}
