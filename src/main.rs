use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::CrosstownBus; 

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

fn main() {
    let mut publisher = CrosstownBus::new_queue_publisher(
        "amqp://guest:guest@localhost:5672".to_owned()
    ).unwrap();

    _ = publisher.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "1".to_owned(), user_name: "2306215816-Amir".to_owned() 
    });
    _ = publisher.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "2".to_owned(), user_name: "2306215816-Budi".to_owned() 
    });
    _ = publisher.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "3".to_owned(), user_name: "2306215816-Cica".to_owned() 
    });
    _ = publisher.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "4".to_owned(), user_name: "2306215816-Dira".to_owned() 
    });
    _ = publisher.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "5".to_owned(), user_name: "2306215816-Emir".to_owned() 
    });
    
    println!("5 events published!");
}