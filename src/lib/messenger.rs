/*
To be used with message broker such as AMQP
the function gets data and send it to a queue
*/

struct MessagePayload<T> {
    pub data: T,
    pub queue_name: String,
}

impl Display for MessagePayload {
    
}
impl MessagePayload {
    pub fn new<T>(data: T, queue_name: &str) -> Self {
        MessagePayload { data, queue_name }
    }

    pub fn send(&self){

    }
}
