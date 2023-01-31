mod md5_hashcash;
mod recover_secret;

use std::net::TcpStream;
use common::request;
use common::structs::{ Message, Subscribe, Challenge, ChallengeTrait, ChallengeResult, ChallengeAnswer };
use md5_hashcash::MD5HashCash;
use recover_secret::RecoverSecret;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    request::send_message(&mut stream, Message::Hello);

    loop {
        let response = request::receive_message(&mut stream);

        match response {
            Message::Welcome(..) => {
                let subscribe = Subscribe { name: "test".to_string() };
                request::send_message(&mut stream, Message::Subscribe(subscribe));
            },
            Message::SubscribeResult(..) => {
                continue;
            },
            Message::PublicLeaderBoard(..) => {
                continue;
            },
            Message::Challenge(challenge) => {
                match challenge {
                    Challenge::MD5HashCash(input) => {
                        let challenge = MD5HashCash::new(input);

                        let output = challenge.solve();

                        let result = Message::ChallengeResult(ChallengeResult {
                            answer: ChallengeAnswer::MD5HashCash(output),
                            next_target: get_next_target()
                        });

                        request::send_message(&mut stream, result);
                    },
                    Challenge::RecoverSecret(input) => {
                        let challenge = RecoverSecret::new(input);

                        let output = challenge.solve();

                        let result = Message::ChallengeResult(ChallengeResult {
                            answer: ChallengeAnswer::RecoverSecret(output),
                            next_target: get_next_target()
                        });

                        request::send_message(&mut stream, result);
                    }
                }
            },
            Message::ChallengeTimeout(..) => {
                break;
            },
            Message::RoundSummary(..) => {
                continue;
            },
            Message::EndOfGame(..) => {
                break;
            },
            _ => {
                panic!("Something went wrong !");
            }
        }
    }
}

fn get_next_target() -> String {
    todo!()
}