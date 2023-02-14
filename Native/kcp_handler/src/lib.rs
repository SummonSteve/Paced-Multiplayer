mod client;
mod frame;
mod server;
use flume::{unbounded, Receiver, Sender};
use frame::{DataFrame, Event};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
struct NetworkKcpServer {
    port: u16,
    max_connection: u16,
    data_frame_tx: (Sender<DataFrame>, Receiver<DataFrame>),
    data_frame_rx: (Sender<DataFrame>, Receiver<DataFrame>),
}

#[methods]
impl NetworkKcpServer {
    fn new(_owner: &Node) -> Self {
        let tx = unbounded();
        let rx = unbounded();
        NetworkKcpServer {
            port: 23231,
            max_connection: 16,
            data_frame_tx: tx,
            data_frame_rx: rx,
        }
    }

    #[method]
    fn configure(&mut self, port: u16, max_connection: u16) {
        self.port = port;
        self.max_connection = max_connection;
    }

    #[method]
    fn start(&self) {
        let port = self.port;
        let max_connection = self.max_connection;
        let tx = self.data_frame_tx.clone();
        let rx = self.data_frame_rx.clone();

        std::thread::spawn(move || {
            server::start(port, max_connection, tx.1, rx.0);
        });

        godot_print!("[Rust Native] Server started")
    }

    #[method]
    fn recv_data_frame(&self) -> Option<PoolArray<u8>> {
        match self.data_frame_rx.1.try_recv() {
            Ok(data_frame) => Some(data_frame.into()),
            Err(_) => None,
        }
    }

    #[method]
    fn send_data_frame(&self, raw_data: PoolArray<u8>) {
        match self.data_frame_tx.0.try_send(raw_data.into()) {
            Err(e) => godot_error!("{}", e),
            _ => (),
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node)]
struct NetworkKcpClient {
    port: u16,
    data_frame_tx: (Sender<DataFrame>, Receiver<DataFrame>),
    data_frame_rx: (Sender<DataFrame>, Receiver<DataFrame>),
}

#[methods]
impl NetworkKcpClient {
    fn new(_owner: &Node) -> Self {
        let tx = unbounded();
        let rx = unbounded();
        NetworkKcpClient {
            port: 23231,
            data_frame_tx: tx,
            data_frame_rx: rx,
        }
    }

    #[method]
    fn start(&self) {
        let port = self.port;
        let tx = self.data_frame_tx.clone();
        let rx = self.data_frame_rx.clone();
        std::thread::spawn(move || client::start(port, tx.1, rx.0));
        godot_print!("[Rust Native] Client started")
    }

    #[method]
    fn recv_data_frame(&self) -> Option<PoolArray<u8>> {
        match self.data_frame_rx.1.try_recv() {
            Ok(data_frame) => Some(data_frame.into()),
            Err(_) => None,
        }
    }

    #[method]
    fn send_data_frame(&self, raw_data: PoolArray<u8>) {
        let data: DataFrame = raw_data.into();
        match self.data_frame_tx.0.try_send(data) {
            Err(e) => godot_print!("{}", e),
            _ => (),
        }
    }
}

#[derive(NativeClass)]
#[inherit(Node)]
struct TestUdpClient {
    test: i32,
}

#[methods]
impl TestUdpClient {
    fn new(_owner: &Node) -> Self {
        TestUdpClient { test: 1 }
    }

    #[method]
    fn _ready(&self) {
        godot_print!("{}", self.test);
    }

    #[method]
    fn set_test(&mut self, value: i32) {
        self.test = value;
        godot_print!("{}", self.test)
    }
}

struct KcpNet;

#[gdnative::init::callbacks]
impl GDNativeCallbacks for KcpNet {
    fn nativescript_init(handle: InitHandle) {
        handle.add_class::<NetworkKcpClient>();
        handle.add_class::<NetworkKcpServer>();
        handle.add_class::<TestUdpClient>();
    }
}
