use futures::stream::StreamExt;
use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{
    io::{self, AsyncBufReadExt},
    select,
    sync::mpsc::{Receiver, UnboundedSender},
};

use tracing_subscriber::EnvFilter;

use crate::event::{AppEvent, Event};

#[derive(NetworkBehaviour)]
struct CustomBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[derive(Default, Debug)]
struct UserInfo {
    name: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Message {
    sender_name: String,
    message: String,
}

// async fn chat() -> Result<(), Box<dyn Error>> {
//     // read full lines from stdin

//     let mut user_info = UserInfo::default();

//     loop {
//         println!("Enter your name:");

//         let mut stdin = io::BufReader::new(io::stdin()).lines();

//         let line = stdin.next_line().await?;

//         match line {
//             Some(name) => {
//                 println!("Your name is '{name}'. Searching for connection...");
//                 user_info.name = name;
//                 break;
//             }
//             _ => {
//                 println!("Invalid name! Try again:");
//             }
//         }
//     }

//     create_and_connect(user_info).await
// }

pub async fn create_and_connect(
    user_name: String,
    mut rx: Receiver<String>,
    event_tx: UnboundedSender<Event>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut user_info = UserInfo::default();
    user_info.name = user_name;
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        // .with_quic()
        .with_behaviour(|key| {
            // content-address messages by taking hash of the message as an id
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict) // enforce message signing
                .message_id_fn(message_id_fn)
                .build()
                .map_err(io::Error::other)?;

            // build a gossipsub network behavior using the config defined above
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(CustomBehaviour { gossipsub, mdns })
        })?
        .build();

    // create a gossipsub topic
    let topic = gossipsub::IdentTopic::new("sam's chat");
    // subscribe the swarm to our topic
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    // listen on all interfaces and OS-assigned port
    // swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // println!("Enter messages via STDIN to send to connected peers using gossipsub");

    // let mut stdin = io::BufReader::new(io::stdin()).lines();

    loop {
        select! {
            Some(line) = rx.recv() => {
                let message = Message { sender_name: user_info.name.clone(), message: line };
                let buf = rmp_serde::to_vec(&message);

                match &buf {
                    Err(e) => println!("Serializer error: {e:?}"),
                    _ => {}
                }

                if let Err(e) = swarm
                    .behaviour_mut().gossipsub
                    .publish(topic.clone(), buf.unwrap()) {
                        println!("Publish error: {e:?}");
                }
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(CustomBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        // println!("mDNS discovered a new peer: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(CustomBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        // println!("mDNS discover peer has expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(CustomBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: _peer_id,
                    message_id: _id,
                    message,
                })) => {
                    let message_struct: Message = rmp_serde::from_slice(&message.data)?;
                    let msg = format!("{}: {}", message_struct.sender_name, message_struct.message);
                    println!("{}", msg.clone());
                    let _ = event_tx.send(Event::App(AppEvent::MessageReceived(msg)));
                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    // println!("Local node is listening on {address}");
                },
                _ => {}
            }
        }
    }
}
