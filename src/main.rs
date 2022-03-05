fn main() {
    let listen_port: u16 = std::env::args().nth(1).expect("no port given").parse::<u16>().unwrap();
    minchain::run(listen_port);
}
