use std::fmt;

use racoon_macros::racoon_debug;

///To be used with message broker such as AMQP
///the function gets data and send it to a queue
#[derive(Debug)]
pub struct MessageQueue<T> {
    pub data: T,
    pub queue_name: String,
}

impl<T: std::fmt::Display> fmt::Display for MessageQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", &self.data, &self.queue_name)
    }
}

impl<T: std::fmt::Display + std::fmt::Debug> MessageQueue<T> {
    pub fn new(data: T, queue_name: &str) -> Self {
        MessageQueue {
            data,
            queue_name: queue_name.to_string(),
        }
    }

    pub fn enqueue(&self) {
        //TODO: add data to queue
        racoon_debug!("added new data to queue", Some(&self.queue_name));
        // println!(" data: {}, queue name: {}", self.data, self.queue_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messenger() {
        let queue_data = String::from("Activation");
        let queue_name = "mailer queue";

        let sample_queue = MessageQueue::new(queue_data, queue_name);
        sample_queue.enqueue();
        // let sample_queue = MessageQueue::new(&queue_data, queue_name);

        assert_eq!(sample_queue.data, String::from("Activation"));
    }
}
