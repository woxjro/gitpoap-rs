pub enum Status {
    Claimed,
    Unclaimed,
    Pending,
    Minting,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Claimed => "claimed".to_string(),
            Status::Unclaimed => "unclaimed".to_string(),
            Status::Pending => "pending".to_string(),
            Status::Minting => "minting".to_string(),
        }
    }
}
