use libp2p::{
    identity, PeerId,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity, Topic},
    kad::{Kademlia, KademliaConfig, KademliaEvent, store::MemoryStore},
    swarm::{SwarmBuilder, NetworkBehaviour, NetworkBehaviourEventProcess},
    tcp::TcpConfig,
    yamux::YamuxConfig,
    noise::{Keypair, NoiseConfig, X25519Spec},
    core::transport::upgrade,
    core::Transport,
};
use async_trait::async_trait;
use crate::config::Config;

#[derive(NetworkBehaviour)]
struct MyBehaviour {
    mdns: Mdns,
    gossipsub: Gossipsub,
    kademlia: Kademlia<MemoryStore>,
}

#[async_trait]
impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(peers) => {
                for (peer_id, _addr) in peers {
                    self.gossipsub.add_explicit_peer(&peer_id);
                }
            }
            MdnsEvent::Expired(peers) => {
                for (peer_id, _addr) in peers {
                    self.gossipsub.remove_explicit_peer(&peer_id);
                }
            }
        }
    }
}

#[async_trait]
impl NetworkBehaviourEventProcess<GossipsubEvent> for MyBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        if let GossipsubEvent::Message { propagation_source: peer_id, message_id: id, message, .. } = event {
            println!(
                "Received message: {:?} from peer: {:?}",
                String::from_utf8_lossy(&message.data),
                peer_id
            );
        }
    }
}

#[async_trait]
impl NetworkBehaviourEventProcess<KademliaEvent> for MyBehaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        if let KademliaEvent::InboundRequest { request, .. } = event {
            println!("Received Kademlia request: {:?}", request);
        }
    }
}

pub async fn start_network(config: &Config) {
    let local_keys = identity::Keypair::generate_ed25519();
    let id_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&local_keys)
        .expect("Failed to create authentic keypair");

    let transport = TcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(id_keys).into_authenticated())
        .multiplex(YamuxConfig::default())
        .boxed();

    let mdns = Mdns::new(MdnsConfig::default()).await.unwrap();
    let gossipsub_config = GossipsubConfig::default();
    let gossipsub = Gossipsub::new(MessageAuthenticity::Signed(local_keys.clone()), gossipsub_config).expect("Failed to create gossipsub");

    let store = MemoryStore::new(PeerId::from(local_keys.public()));
    let mut kademlia = Kademlia::with_config(PeerId::from(local_keys.public()), store, KademliaConfig::default());

    let behaviour = MyBehaviour { mdns, gossipsub, kademlia };

    let mut swarm = SwarmBuilder::new(transport, behaviour, PeerId::from(local_keys.public()))
        .executor(Box::new(|fut| { tokio::spawn(fut); }))
        .build();

    let topic = Topic::new("example-topic");
    swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();

    loop {
        match swarm.next().await.unwrap() {
            libp2p::swarm::SwarmEvent::Behaviour(e) => match e {
                MyBehaviourEvent::Gossipsub(GossipsubEvent::Message {
                    propagation_source,
                    message_id,
                    message,
                }) => {
                    println!(
                        "Received message: {:?} from peer: {:?}",
                        String::from_utf8_lossy(&message.data),
                        propagation_source
                    );
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub async fn announce_content(content_id: &str) {
    let local_keys = identity::Keypair::generate_ed25519();
    let id_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&local_keys)
        .expect("Failed to create authentic keypair");

    let transport = TcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(id_keys).into_authenticated())
        .multiplex(YamuxConfig::default())
        .boxed();

    let mdns = Mdns::new(MdnsConfig::default()).await.unwrap();
    let gossipsub_config = GossipsubConfig::default();
    let mut gossipsub = Gossipsub::new(MessageAuthenticity::Signed(local_keys.clone()), gossipsub_config).expect("Failed to create gossipsub");

    let store = MemoryStore::new(PeerId::from(local_keys.public()));
    let mut kademlia = Kademlia::with_config(PeerId::from(local_keys.public()), store, KademliaConfig::default());

    let behaviour = MyBehaviour { mdns, gossipsub, kademlia };

    let mut swarm = SwarmBuilder::new(transport, behaviour, PeerId::from(local_keys.public()))
        .executor(Box::new(|fut| { tokio::spawn(fut); }))
        .build();

    let topic = Topic::new("example-topic");
    swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();

    let key = content_id.as_bytes();
    swarm.behaviour_mut().kademlia.put_record(libp2p::kad::record::Record::new(key.to_vec(), "content".into()), libp2p::kad::Quorum::One).unwrap();

    loop {
        match swarm.next().await.unwrap() {
            libp2p::swarm::SwarmEvent::Behaviour(e) => match e {
                MyBehaviourEvent::Gossipsub(GossipsubEvent::Message {
                    propagation_source,
                    message_id,
                    message,
                }) => {
                    println!(
                        "Received message: {:?} from peer: {:?}",
                        String::from_utf8_lossy(&message.data),
                        propagation_source
                    );
                }
                MyBehaviourEvent::Kademlia(KademliaEvent::OutboundQueryCompleted { result, .. }) => {
                    match result {
                        libp2p::kad::QueryResult::PutRecord(Ok(_)) => {
                            println!("Content announced: {}", content_id);
                        }
                        libp2p::kad::QueryResult::PutRecord(Err(e)) => {
                            println!("Failed to announce content: {:?}", e);
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
