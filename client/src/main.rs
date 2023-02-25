mod md5_hashcash;
mod recover_secret;
use common::request;
use common::structs::{
    Challenge, ChallengeAnswer, ChallengeResult, ChallengeTrait, Message, PublicLeaderBoard,
    Subscribe,
};
use md5_hashcash::MD5HashCash;
use recover_secret::RecoverSecret;
use std::env;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();
    let address;

    if args.len() != 2 {
        address = "127.0.0.1:7878";
    } else {
        address = &args[1];
    }

    let mut stream = TcpStream::connect(address).unwrap();

    let player_name: String = "Groupe 5".to_string();
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
