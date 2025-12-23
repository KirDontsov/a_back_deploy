pub mod ai_processing_consumer;
pub mod consumer;
pub mod message;

pub use self::ai_processing_consumer::start_ai_processing_consumer;
pub use self::consumer::start_rabbitmq_consumer;
