extern crate indigestion;
extern crate getopts;

use indigestion::proxy::ProxyConfig;
use getopts::{optopt,getopts,optmulti,optflag,OptGroup};
use std::os;

fn usage(opts: &[OptGroup], error: bool) {
    println!("{}", getopts::usage("test", opts));
    if error {
        os::set_exit_status(1);
    }
}


fn setup_proxy(opts: getopts::Matches) -> ProxyConfig {
    let config = ProxyConfig::new(("localhost".to_string(), 3000),
                                  ("localhost".to_string(), from_str::<u16>(opts.opt_str("l").unwrap().as_slice()).unwrap()));

    opts.

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

    let proxy = config.connect();
}
