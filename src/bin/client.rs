use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};

use std::{net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::{Duration, SystemTime, UNIX_EPOCH}};

fn main() {
    let mut client = RenetClient::new(ConnectionConfig::default());

    let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("time should go forward").as_nanos();
    let id = (time >> 64) as u64 ^ (time & u64::MAX as u128) as u64;

    // Setup transport layer using renet_netcode
    const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000);
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let authentication = ClientAuthentication::Unsecure {
        server_addr: SERVER_ADDR,
        client_id: id,
        user_data: None,
        protocol_id: 0,
    };

    println!("Id {}", id);

    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    // Your gameplay loop
    loop {
        let delta_time = Duration::from_millis(16);
        // Receive new messages and update client
        client.update(delta_time);
        transport.update(delta_time, &mut client).unwrap();
        
        if client.is_connected() {
            // Receive message from server
            while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
                // Handle received message
            }
            
            // Send message
            client.send_message(DefaultChannel::ReliableOrdered, "client text");
        }
    
        // Send packets to server using the transport layer
        transport.send_packets(&mut client);
        
        std::thread::sleep(delta_time); // Running at 60hz
    }
}