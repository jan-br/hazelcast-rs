use std::collections::HashMap;
use crate::protocol::client_message::ClientMessage;

pub struct FragmentedClientMessageHandler {
  pub fragmented_messages: HashMap<i64, ClientMessage>,
}

impl FragmentedClientMessageHandler {
  pub fn new() -> Self {
    FragmentedClientMessageHandler {
      fragmented_messages: HashMap::new(),
    }
  }
  pub async fn handle_fragmented_message(&mut self, mut client_message: ClientMessage) {
    let fragmentation_frame = client_message.start_frame.as_ref().unwrap().clone();
    let fragmentation_id = client_message.get_fragmentation_id().await;
    client_message.drop_fragmentation_frame().await;
    if fragmentation_frame.has_begin_fragment_flag().await {
      self.fragmented_messages.insert(fragmentation_id, client_message);
    } else {
      let existing_message = self.fragmented_messages.get_mut(&fragmentation_id);
      if existing_message.is_none() {
        todo!("handle softerror");
      }
      existing_message.unwrap().merge(client_message);
      if fragmentation_frame.has_end_fragment_flag().await {
        self.fragmented_messages.remove(&fragmentation_id);
      }
    }
  }
}