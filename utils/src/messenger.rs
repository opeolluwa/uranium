use std::fmt;

///To be used with message broker such as AMQP
///the function gets data and send it to a queue
#[derive(Debug)]
struct MessagePayload<T> {
    pub data: T,
    pub queue_name: String,
}

impl<T: std::fmt::Display> fmt::Display for MessagePayload<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", &self.data, &self.queue_name)
    }
}

impl<T: std::fmt::Display> MessagePayload<T> {
    pub fn new(data: T, queue_name: &str) -> Self {
        MessagePayload {
            data,
            queue_name: queue_name.to_string(),
        }
    }

    pub fn enqueue(&self) {
        //TODO: add data to queue
        println!(" data: {}, queue name: {}", &self.data, &self.queue_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messenger() {
        let queue_data = String::from("Activation");
        let queue_name = "mailer queue";

        let sample_queue = MessagePayload::new(queue_data, queue_name);
        sample_queue.enqueue();
        // let sample_queue = MessagePayload::new(&queue_data, queue_name);

        assert_eq!(sample_queue.data, String::from("Activation"));
    }
}
