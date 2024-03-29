#[derive(Debug)]
enum Gender {
    #[allow(dead_code)]
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);


#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => {
            println!("user {:?} joined", uid);
        }
        Event::Leave((uid, tid)) => {
            println!("user {:?} left {:?}", uid, tid);
        }
        Event::Message((_, _, msg)) => {
            println!("broadcast: {}", msg);
        }
    }
}

// 只关心 Event::message
fn process_event_if_let(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

fn main() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };

    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Female,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };

    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Leave((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello World!".into()));

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );

    // pattern match event
    process_event(&event1);
    process_event(&event2);
    process_event(&event3);
}