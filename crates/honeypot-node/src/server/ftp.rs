use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncBufReadExt,BufReader,AsyncWriteExt};
struct Session{
    username: Option<String>,
    authenticated: bool,
    cwd: String,
    passive_listener:Option<String>,
    ipadd: String,
}
enum Command{
    User(String),
    Pass(String),
    List,
    Retr(String),
    Stor(String),
    Pasv,
    Quit,
    Unknown,
}
fn parse_command(line:&str)->Command {
    let part:Vec<&str> = line.split_whitespace().collect();
    match part.get(0){
        Some(&"User")=>Command::User(part.get(1).unwrap_or(&"").to_string()),
        Some(&"Pass")=>Command::Pass(part.get(1).unwrap_or(&"").to_string()),
        Some(&"List")=>Command::List,
        Some(&"Retr")=>Command::Retr(part.get(1).unwrap_or(&"").to_string()),
        Some(&"Stor")=>Command::Stor(part.get(1).unwrap_or(&"").to_string()),
        Some(&"Pasv")=>Command::Pasv,
        Some(&"Quit")=>Command::Quit,
        _ => Command::Unknown,
    }
}
async fn list_file()-> String{
    "drwxr-xr-x 1 root root 4096 etc\r\n-rw-r--r-- 1 root root 123 passwd\r\n".to_string()
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
            Command::User(u)=> {
                session.username=Some(u.clone());
                let msg=format!("Registered the username as {}",u);
                writter.write_all(msg.as_bytes()).await.unwrap();
            },
            Command::Pass(p)=>{
                session.authenticated=true;
                writter.write_all(b"authenticated inside the ftp server\r\n").await.unwrap();
            },
            Command::List=>{
                let file_system=list_file().await;
                writter.write_all(list_file().await.as_bytes()).await.unwrap();
            }
            Command::Quit=>{
                writter.write_all(b"Exited the ftp server\r\n").await.unwrap();
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
    let listener=TcpListener::bind("0.0.0.0:2121").await.unwrap();
    println!("Executing ftp");
    loop {
        let (stream,addr) = listener.accept().await.unwrap();
        tokio::spawn(handle_client(stream,addr));
    }
}
