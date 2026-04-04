use hyper::{Body,Request,Response,Server};
use hyper::service::{make_service_fn,service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle(req: Request<Body> , client_ip:SocketAddr) -> Result<Response<Body>,Infallible>{
    println!("IP: {} , Method: {} , Uri: {} ",client_ip,req.method(),req.uri());
    if req.uri().path()=="/login"{
        return Ok(Response::new(Body::from("<h1>Login Page</h1><form method='POST'></form>")));
    }
    Ok(Response::new(Body::from("<h1>Welcome to webserver</h1>")))
}
pub async fn start_http(){
    let addr=([0,0,0,0],8084).into();
    print!("{}",addr);
    let make_svc=make_service_fn(|conn: &hyper::server::conn::AddrStream|{
        let remoteaddr=conn.remote_addr();
        async move{
            Ok::<_,Infallible>(service_fn(move|req|{
                handle(req,remoteaddr)
            }))
        }
    });
    let server=Server::bind(&addr).serve(make_svc);
    println!("The honeypot is running !!");
    if let Err(e)= server.await {
        eprintln!("Server Error!: {}",e);
    }
}
