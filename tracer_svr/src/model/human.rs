use serde::{Deserialize, Serialize};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::crud;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Human {
    pub id: Option<u32>,
    pub create_time: Option<FastDateTime>,

    pub name: Option<String>,
}
crud!(Human{}, "human");