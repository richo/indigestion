use std::sync::Arc;
use std::io::{Listener,Acceptor};
use std::io::{TcpListener,TcpStream};
use std::comm::channel;
use std::str;

pub type Endpoint = (String, u16);

#[deriving(Show)]
pub struct ProxyConfig {
    osock: Endpoint,
    isock: Endpoint,
    ipeeks: Vec<Endpoint>,
    opeeks: Vec<Endpoint>,
}

pub struct Proxy {
    osock: TcpStream,
    isock: TcpStream,
    ipeeks: Vec<TcpStream>,
    opeeks: Vec<TcpStream>,
}

impl Proxy {

    pub fn run(self) {
        type Data = [u8, ..1024];

        fn run<R: Reader, W:Writer>(mut from: R, mut to: W, mut taps: Vec<W>) {
            debug!("Entering runloop");
            loop {
                let mut buf = [0, ..1024];
                match from.read(buf) {
                    Err(_) => {
                        println!("Input socket closed");
                        return;
                    }
                    _ => { /* Everything is probably ok!" */ }
                }
                // XXX hack
                let s = str::from_utf8(buf);
                debug!("< {}", s);
                // /XXX
                for tap in taps.mut_iter() {
                    (tap).write(buf).ok();
                }
                to.write(buf).ok();
            }
        }

        let (isock, osock, ipeeks, opeeks) =
            (self.isock, self.osock, self.ipeeks, self.opeeks);
        spawn(proc() {run(isock, osock, ipeeks); })
        // spawn(proc() { run(self.osock, self.isock, self.opeeks); });
        // spawn(proc() { run(self.isock, self.osock, self.ipeeks); });
    }
}

impl ProxyConfig {
    pub fn new(output: Endpoint, input: Endpoint) -> ProxyConfig {
        ProxyConfig {
            osock: output,
            isock: input,
            ipeeks: vec![],
            opeeks: vec![],
        }
    }

    pub fn add_ipeek(&mut self, endpoint: Endpoint) {
        self.ipeeks.push(endpoint);
    }

    pub fn add_opeek(&mut self, endpoint: Endpoint) {
        self.opeeks.push(endpoint);
    }

    pub fn connect(&self) -> Proxy {
        let osock = match self.osock {
            (ref host, port) => TcpStream::connect(host.as_slice(), port).ok()
                .expect(format!("Couldn't connect to {}:{}", host, port).as_slice())
        };

        let isock = match self.isock {
            (ref host, port) => {
                let listener = TcpListener::bind(host.as_slice(), port).ok()
                    .expect(format!("Couldn't bind to {}:{}", host, port).as_slice());
                listener.listen().accept().ok().expect("Couldn't accept connection")
            }
        };

        let ipeeks = self.ipeeks.iter().map(|e: &Endpoint| -> TcpStream {
            match *e { (ref host, port) => TcpStream::connect(host.as_slice(), port).unwrap() }
        }).collect();

        // TODO opeeks

        Proxy {
            osock: (osock),
            isock: (isock),
            ipeeks: (ipeeks),
            opeeks: (vec![]),
        }
    }
}
