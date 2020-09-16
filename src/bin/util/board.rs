


use crate::util::{
    StatefulList,
    message::Message,
};




pub struct ChatBoard {
    entries: StatefulList<Message>
}


impl ChatBoard {

    pub fn new() -> ChatBoard {

        ChatBoard {
            entries: StatefulList::with_items(vec![
                Message::new("TENX", "hello") ,
                Message::new("Alice", "Hi, there") ,
            ])
        }

    }

    pub fn entries(&mut self) -> &mut StatefulList<Message> {
        &mut self.entries
    }

}
