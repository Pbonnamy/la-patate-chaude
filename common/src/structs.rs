use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    ChallengeTimeout(ChallengeTimeout),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hello;

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    pub version: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard (Vec<PublicPlayer>);

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeTimeout {
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { 
        used_time: f64, 
        next_target: String 
    },
    Ok { 
        used_time: f64, 
        next_target: String 
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    pub leader_board: PublicLeaderBoard
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
    RecoverSecret(RecoverSecretInput),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    RecoverSecret(RecoverSecretOutput),
}

// MD5HashCash
#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

// RecoverSecret
#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}