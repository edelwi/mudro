use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LimitOffset {
    pub limit: Option<u16>,
    pub offset: Option<u16>,
}
