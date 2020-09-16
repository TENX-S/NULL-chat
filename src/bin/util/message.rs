


use chrono::Local;
use std::sync::Arc;
use crate::util::user::User;


pub struct Message {
    from: Arc<User>,
    to: Arc<Vec<User,>>,
    time: Arc<String>,
    content: Arc<String>,
}


impl Message {

    pub fn new(user_name: &str, content:  &str) -> Message {

        Message {
            from: Arc::new({
                let mut user = User::default();
                user.set_name(user_name);
                user
            }),
            to: Arc::new(Vec::<User>::new()),
            time: Arc::new({
                    let now = Local::now();
                    format!("{} {} UTC{}", now.date().format("%v"), now.time().format("%X"), now.offset())
                }),
            content: Arc::new(content.into()),
        }

    }


    pub fn to(&self) -> Arc<Vec<User>> {
        Arc::clone(&self.to)
    }

    pub fn time(&self) -> Arc<String> {
        Arc::clone(&self.time)
    }

    pub fn val(&self) -> Arc<String> {
        Arc::clone(&self.content)
    }

    pub fn sender_name(&self) -> Arc<String> {
        Arc::clone(&self.from.name())
    }

    pub fn recv_name(&self) -> Arc<Vec<User>> {
        Arc::clone(&self.to)
    }

}
