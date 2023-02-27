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

/// It connects to the server and handle the responses
/// # Arguments
/// * `ip:port` - The ip and port of the server
/// # Example
/// ```
/// cargo run 127.0.0.1:7878
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();
    let address;

    if args.len() != 2 {
        address = "127.0.0.1:7878";
    } else {
        address = &args[1];
    }

    // Connect to the server
    let mut stream = TcpStream::connect(address).unwrap();

    let player_name: String = "Groupe 5".to_string();
    let mut leaderboard: PublicLeaderBoard = PublicLeaderBoard(Vec::new());

    request::send_message(&mut stream, Message::Hello);

    loop {
        let response = request::receive_message(&mut stream);

        // Pattern matching on the response
        match response {
            Message::Welcome(..) => {
                let subscribe = Subscribe {
                    name: player_name.clone(),
                };
                request::send_message(&mut stream, Message::Subscribe(subscribe));
            }
            Message::SubscribeResult(..) => {
                println!(" \n\x1b[32mSucessfully subscribed\x1b[0m");
                continue;
            }
            Message::PublicLeaderBoard(input) => {
                leaderboard = input;
            }
            //Pattern matching on the challenge
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
                println!(" \n\x1b[32mTimed out\x1b[0m");
                break;
            }
            Message::RoundSummary(..) => {
                println!(" \n\x1b[32mEnd of round\x1b[0m");
                continue;
            }
            Message::EndOfGame(..) => {
                println!(" \n\x1b[32mEnd of game\x1b[0m");
                break;
            }
            _ => {
                panic!("Something went wrong !");
            }
        }
    }
}

/// It returns the name of a random player (which is not the actual player)
/// # Arguments
/// * `leaderboard` - A PublicLeaderBoard struct
/// * `player_name` - A string containing the name of the actual player
/// # Example
/// ```
/// let player_name = get_next_target(&leaderboard, &player_name);
/// ```
/// # Output
/// A string containing the name of the next target
fn get_next_target(leaderboard: &PublicLeaderBoard, player_name: &String) -> String {
    // Get a random player
    let random_player = leaderboard
        .0
        .iter()
        .find(|player| player.name != *player_name);

    // If no player found return an empty string
    return match random_player {
        Some(player) => player.name.clone(),
        None => return String::new(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::structs::PublicPlayer;

    #[test]
    fn test_get_next_target() {
        let leaderboard = PublicLeaderBoard(vec![
            PublicPlayer {
                name: "Player 1".to_string(),
                stream_id: "stream_id".to_string(),
                score: 0,
                steps: 0,
                is_active: false,
                total_used_time: 0.0,
            },
            PublicPlayer {
                name: "Player 2".to_string(),
                stream_id: "stream_id".to_string(),
                score: 0,
                steps: 0,
                is_active: false,
                total_used_time: 0.0,
            },
            PublicPlayer {
                name: "Player 3".to_string(),
                stream_id: "stream_id".to_string(),
                score: 0,
                steps: 0,
                is_active: false,
                total_used_time: 0.0,
            },
        ]);

        let player_name = "Player 1".to_string();

        let next_target = get_next_target(&leaderboard, &player_name);

        assert_eq!(next_target, "Player 2");
    }

    #[test]
    fn test_get_next_target_with_one_player() {
        let leaderboard = PublicLeaderBoard(vec![PublicPlayer {
            name: "Player 1".to_string(),
            stream_id: "stream_id".to_string(),
            score: 0,
            steps: 0,
            is_active: false,
            total_used_time: 0.0,
        }]);

        let player_name = "Player 1".to_string();

        let next_target = get_next_target(&leaderboard, &player_name);

        assert_eq!(next_target, "");
    }
}
