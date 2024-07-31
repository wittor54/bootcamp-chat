use candid::CandidType;

#[derive(Clone, CandidType)]

pub struct userdata {
    nickname: String,
    avatar_url: Option<String>,
}

impl userdata {
    pub fn new(nickname: String) -> Self {
        Self {
            nickname,
            avatar_url: None,
        }
    }
}
