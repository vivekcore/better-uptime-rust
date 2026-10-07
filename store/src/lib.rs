pub struct Store {}

impl Store {
    pub fn create_user(&self) {
        print!("create user fn called")
    }
    pub fn create_website(&self) -> String {
        String::from("1")
    }
    pub fn get_website(&self) {
        print!("get website fn called")
    }
}