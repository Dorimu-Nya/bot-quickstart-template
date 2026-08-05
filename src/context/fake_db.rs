pub struct FakeDb;

impl FakeDb {
    pub fn get_profile(&self, user_id: &str) -> String {
        format!("User:{user_id}")
    }
}
