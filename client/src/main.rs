mod md5_hashcash;
mod recover_secret;

use common::request;
use common::structs::{
    Challenge, ChallengeAnswer, ChallengeResult, ChallengeTrait, Message, PublicLeaderBoard,
    Subscribe,
};
use md5_hashcash::MD5HashCash;
use rand::{distributions::Alphanumeric, Rng};
use recover_secret::RecoverSecret;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let player_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    let mut leaderboard: PublicLeaderBoard = PublicLeaderBoard(Vec::new());

    request::send_message(&mut stream, Message::Hello);

    loop {
        let response = request::receive_message(&mut stream);

        match response {
            Message::Welcome(..) => {
                let subscribe = Subscribe {
                    name: player_name.clone(),
                };
                request::send_message(&mut stream, Message::Subscribe(subscribe));
            }
            Message::SubscribeResult(..) => {
                continue;
            }
            Message::PublicLeaderBoard(input) => {
                leaderboard = input;
            }
            Message::Challenge(challenge) => match challenge {
                Challenge::MD5HashCash(input) => {
                    let challenge = MD5HashCash::new(input);

                    let output = challenge.solve();

                    let result = Message::ChallengeResult(ChallengeResult {
                        answer: ChallengeAnswer::MD5HashCash(output),
                        next_target: get_next_target(&leaderboard, &player_name),
                    });

                    request::send_message(&mut stream, result);
                }
                Challenge::RecoverSecret(input) => {
                    let challenge = RecoverSecret::new(input);

                    let output = challenge.solve();

                    let result = Message::ChallengeResult(ChallengeResult {
                        answer: ChallengeAnswer::RecoverSecret(output),
                        next_target: get_next_target(&leaderboard, &player_name),
                    });

                    request::send_message(&mut stream, result);
                }
            },
            Message::ChallengeTimeout(..) => {
                break;
            }
            Message::RoundSummary(..) => {
                continue;
            }
            Message::EndOfGame(..) => {
                break;
            }
            _ => {
                panic!("Something went wrong !");
            }
        }
    }
}

fn get_next_target(leaderboard: &PublicLeaderBoard, player_name: &String) -> String {
    let random_player = leaderboard
        .0
        .iter()
        .find(|player| player.name != *player_name);

    return match random_player {
        Some(player) => player.name.clone(),
        None => return String::new(),
    };
}
