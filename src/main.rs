use std::error::Error;

use libp2p::{
    identity,
    PeerId,
    futures::StreamExt,
    // mdns(Multicast DNS) 是一种协议，将主机名解析为IP地址的协议
    // https://datatracker.ietf.org/doc/html/rfc6762
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 创建密钥对
    let id_keys = identity::Keypair::generate_ed25519();
    
    // 使用公钥创建peer id
    let peer_id = PeerId::from(id_keys.public());
    
    // 使用密钥对创建传输
    let transport = libp2p::development_transport(id_keys).await?;

    // 创建网络行为， 简单的keep alive(收到ping后回复pong)
    let behaviour = Mdns::new(MdnsConfig::default()).await?;

    // 使用网络行为和传输创建swarm
    let mut swarm = Swarm::new(transport, behaviour, peer_id);

    // 启动swarm监听本地所有IP的随机端口
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // 事件监听器
    loop {
        match swarm.select_next_some().await {
            
            // 监听listenAddr事件
            SwarmEvent::NewListenAddr { listener_id, address } => {
                println!("Listening on local address {:?} - {}", listener_id, address)
            }
            
            // mdns::发现节点事件
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr)
                }
            }

            // mdns::节点过期事件
            SwarmEvent::Behaviour(MdnsEvent::Expired(peers)) => {
                for (peer, addr) in peers {
                    println!("expired {} {}", peer, addr)
                }
            }
            _ => {}
        }
    }
}
