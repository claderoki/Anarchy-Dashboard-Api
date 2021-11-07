use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct PollOption {
    positive: bool,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    id: i32,
    question: String,
    channel_id: u64,
    result_channel_id: Option<u64>,
    pin: bool,
    mention_role: bool,
    delete_after_results: bool,
    custom: bool,
    role_id_needed: Option<u64>,
    vote_percentage_needed_to_pass: i16,
    max_votes_per_user: i16,
    options: Vec<PollOption>,
}