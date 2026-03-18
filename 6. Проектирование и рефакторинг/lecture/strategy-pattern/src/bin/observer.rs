trait Observer {
    fn update(&self, event: &str);
}

struct EventPublisher {
    observers: Vec<Box<dyn Observer>>,
}

impl EventPublisher {
    fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    fn subscribe(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify(&self, event: &str) {
        for observer in &self.observers {
            observer.update(event);
        }
    }
}

struct LogObserver {
    name: String,
}

impl Observer for LogObserver {
    fn update(&self, event: &str) {
        println!("[{}] Event: {}", self.name, event);
    }
}

fn main() {
    let mut publisher = EventPublisher::new();

    publisher.subscribe(Box::new(LogObserver {
        name: "Logger1".to_string(),
    }));

    publisher.subscribe(Box::new(LogObserver {
        name: "Logger2".to_string(),
    }));

    publisher.notify("User logged in");
    publisher.notify("Data saved");
}
