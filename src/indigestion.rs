extern crate indigestion;
extern crate getopts;

use indigestion::proxy::{ProxyConfig,Endpoint};
use getopts::{optopt,getopts,optmulti,optflag,OptGroup};
use std::os;

fn usage(opts: &[OptGroup], error: bool) {
    println!("{}", getopts::usage("test", opts));
    if error {
        os::set_exit_status(1);
    }
}

fn opt_to_port(o: &str) -> u16 {
    from_str::<u16>(o.as_slice()).unwrap()
}


fn parse_target(target: String) -> Endpoint {
    let parts: Vec<&str> = target.as_slice().split(':').collect();
    return match parts.len() {
        0 => unreachable!(),
        1 => ("localhost".to_string(), opt_to_port(parts[0])),
        2 => (from_str(parts[0]).unwrap(), opt_to_port(parts[1])),
        _ => fail!("Failed to parse: {}", target),
    }
}

fn setup_proxy(opts: getopts::Matches) -> ProxyConfig {
    let endpoint = parse_target(opts.opt_str("t").unwrap());
    println!("{}", endpoint);
    let config = ProxyConfig::new(parse_target(opts.opt_str("t").unwrap()),
                                  ("127.0.0.1".to_string(), opt_to_port(opts.opt_str("l").unwrap().as_slice())));

    println!("{}", config);

    config
}


fn main() {
    let args = os::args();

    let opts = [
        optflag("h", "help", "print this help"),
        optopt("t", "", "set target hostname:port", "TARGET"),
        optopt("l", "listen", "set local listener port for connection", "LISTEN"),
        optmulti("", "ipeek", "set an inbound peek listener port", "IPEEK"),
        optmulti("", "opeek", "set an outbound peek listener port", "OPEEK"),
    ];

    if args.len() == 1 {
        usage(opts, true);
        return;
    }

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };

    if matches.opt_present("h") {
        usage(opts, false);
        return;
    }

    let config = setup_proxy(matches);

    let mut proxy = config.connect();

    proxy.run();
}
