use crate::resources::user::UserObject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Users {
    pub users: Vec<UserObject>,
}
