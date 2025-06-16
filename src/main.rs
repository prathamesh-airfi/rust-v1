#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let message_1 = ChatMessage::<&str> {
        content: "Hello Dear",
        time: String::from("12:00 am"),
    };

    let message_2 = ChatMessage::<String> {
        content: String::from("Get Lost you bastard"),
        time: String::from("12:01 am"),
    };

    let audio_message = ChatMessage::<DigitalContent> {
        content: DigitalContent::AudioFile,
        time: String::from("12:02 am"),
    };

    audio_message.consume_entertainment();

    println!("{}", message_1.retrieve_time());
    println!("{}", message_2.retrieve_time());
    println!("{}", audio_message.retrieve_time());
}
