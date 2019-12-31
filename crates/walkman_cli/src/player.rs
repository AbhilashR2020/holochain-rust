use holochain_walkman_types::{Cassette, WalkmanEvent, WalkmanLogItem, WalkmanSim2hEvent};
use in_stream::InStream;
use lib3h_protocol::data_types::Opaque;
use sim2h::{crypto::SignedWireMessage, wire_message::WireMessage};
use sim2h_client::Sim2hClient;
use std::collections::{hash_map::Entry, HashMap};
use url2::Url2;
use std::convert::TryInto;

#[derive(Default)]
pub struct Sim2hCassettePlayer {}

impl Sim2hCassettePlayer {
    pub fn playback(sim2h_url: &Url2, cassette: Cassette) {
        let mut clients: HashMap<String, Sim2hClient> = HashMap::new();
        for event in cassette.events() {
            match event {
                WalkmanLogItem {
                    time: _,
                    event: WalkmanEvent::Sim2hEvent(event),
                } => match event {
                    WalkmanSim2hEvent::Connect(client_url) => match clients
                        .entry(client_url.clone())
                    {
                        Entry::Vacant(e) => {
                            e.insert(
                                Sim2hClient::new(sim2h_url).expect("Couldn't create sim2h client"),
                            );
                        }
                        Entry::Occupied(_) => {
                            panic!(format!("Tried to connect from url twice: {}", client_url))
                        }
                    },
                    WalkmanSim2hEvent::Disconnect(client_url) => {
                        match clients.entry(client_url.clone()) {
                            Entry::Occupied(e) => {
                                e.remove_entry();
                            }
                            Entry::Vacant(_) => panic!(format!(
                                "Tried to disconnect from url without being connected: {}",
                                client_url
                            )),
                        }
                    }
                    WalkmanSim2hEvent::Message(client_url, message_str) => clients
                        .get_mut(client_url)
                        .map(|client| {
                            // The Sim2hClient was created with a random keypair, but we are going to bypass that agent here
                            // and directly send a saved signed message from a different prior Agent
                            let msg: SignedWireMessage = serde_json::from_str(message_str)
                                .expect("Couldn't parse serialized SignedWireMessage");
                            let wire_msg: WireMessage = msg.payload.clone().try_into().unwrap();
                            println!("Playback WireMessage: {:?}", wire_msg);
                            let to_send: Opaque = msg.into();
                            client
                                .connection()
                                .write(to_send.as_bytes().into())
                                .unwrap();
                        })
                        .unwrap_or_else(|| {
                            panic!("Trying to send message without a client connection")
                        }),
                },
            }
        }
    }
}