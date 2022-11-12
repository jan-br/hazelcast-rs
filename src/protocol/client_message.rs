use std::sync::Arc;
use async_recursion::async_recursion;
use tokio::sync::Mutex;
use crate::codec_builtin::fix_sized_types_codec::FixSizedTypesCodec;
use crate::network::connection::Connection;
use crate::util::bits_util::BitsUtil;

#[derive(Clone)]
pub struct Frame {
  pub content: Arc<Mutex<Vec<u8>>>,
  pub next: Arc<Mutex<Option<Box<Frame>>>>,
  pub flags: Arc<Mutex<i32>>,
}

impl Frame {
  pub const MESSAGE_TYPE_OFFSET: i32 = 0;
  pub const CORRELATION_ID_OFFSET: i32 = Self::MESSAGE_TYPE_OFFSET + BitsUtil::INT_SIZE_IN_BYTES;
  /** @internal */
  pub const RESPONSE_BACKUP_ACKS_OFFSET: i32 = Self::CORRELATION_ID_OFFSET + BitsUtil::LONG_SIZE_IN_BYTES;
  /** @internal */
  pub const PARTITION_ID_OFFSET: i32 = Self::CORRELATION_ID_OFFSET + BitsUtil::LONG_SIZE_IN_BYTES;
  pub const FRAGMENTATION_ID_OFFSET: i32 = 0;

  /** @internal */
  pub const DEFAULT_FLAGS: i32 = 0;
  pub const BEGIN_FRAGMENT_FLAG: i32 = 1 << 15;
  pub const END_FRAGMENT_FLAG: i32 = 1 << 14;
  pub const UNFRAGMENTED_MESSAGE: i32 = Self::BEGIN_FRAGMENT_FLAG | Self::END_FRAGMENT_FLAG;
  pub const IS_FINAL_FLAG: i32 = 1 << 13;
  pub const BEGIN_DATA_STRUCTURE_FLAG: i32 = 1 << 12;
  pub const END_DATA_STRUCTURE_FLAG: i32 = 1 << 11;
  pub const IS_NULL_FLAG: i32 = 1 << 10;
  pub const IS_EVENT_FLAG: i32 = 1 << 9;
  /** @internal */
  pub const IS_BACKUP_AWARE_FLAG: i32 = 1 << 8;
  pub const IS_BACKUP_EVENT_FLAG: i32 = 1 << 7;

  pub const SIZE_OF_FRAME_LENGTH_AND_FLAGS: i32 = BitsUtil::INT_SIZE_IN_BYTES + BitsUtil::SHORT_SIZE_IN_BYTES;

  pub fn new_null_frame() -> Frame {
    Frame::new(vec![], Self::IS_NULL_FLAG)
  }

  pub fn new_begin_frame() -> Frame {
    Frame::new(vec![], Self::BEGIN_DATA_STRUCTURE_FLAG)
  }

  pub fn new_end_frame() -> Frame {
    Frame::new(vec![], Self::END_DATA_STRUCTURE_FLAG)
  }

  pub fn new_default_flags(content: Vec<u8>) -> Self {
    Self {
      content: Arc::new(Mutex::new(content)),
      next: Arc::new(Mutex::new(None)),
      flags: Arc::new(Mutex::new(Self::DEFAULT_FLAGS)),
    }
  }

  pub fn new(content: Vec<u8>, flags: i32) -> Self {
    Self {
      content: Arc::new(Mutex::new(content)),
      flags: Arc::new(Mutex::new(flags)),
      next: Arc::new(Mutex::new(None)),
    }
  }

  pub fn create_initial_frame(size: usize, flags: Option<i32>) -> Self {
    Self::new(vec![0; size], flags.unwrap_or(Self::UNFRAGMENTED_MESSAGE))
  }

  pub async fn get_length(&self) -> i32 {
    let content = self.content.lock().await;
    Self::SIZE_OF_FRAME_LENGTH_AND_FLAGS + content.len() as i32
  }

  pub fn copy(&self) -> Frame {
    self.clone()
  }

  #[async_recursion]
  pub async fn deep_copy(&self) -> Frame {
    let content = self.content.lock().await.clone();
    let flags = self.flags.lock().await.clone();
    let next = self.next.lock().await.clone();
    let next = if let Some(next) = next {
      Some(Box::new(next.deep_copy().await))
    } else {
      None
    };
    Frame {
      content: Arc::new(Mutex::new(content)),
      flags: Arc::new(Mutex::new(flags)),
      next: Arc::new(Mutex::new(next)),
    }
  }

  pub async fn is_begin_frame(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::BEGIN_DATA_STRUCTURE_FLAG)
  }

  pub async fn is_end_frame(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::END_DATA_STRUCTURE_FLAG)
  }

  pub async fn is_null_frame(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::IS_NULL_FLAG)
  }

  pub async fn has_event_flag(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::IS_EVENT_FLAG)
  }

  pub async fn has_backup_event_flag(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::IS_BACKUP_EVENT_FLAG)
  }

  pub async fn is_final_frame(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::IS_FINAL_FLAG)
  }

  pub async fn has_unfragmented_message_flag(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::UNFRAGMENTED_MESSAGE)
  }

  pub async fn has_begin_fragment_flag(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::BEGIN_FRAGMENT_FLAG)
  }

  pub async fn has_end_fragment_flag(&self) -> bool {
    let flags = self.flags.lock().await;
    Self::is_flag_set(*flags, Self::END_FRAGMENT_FLAG)
  }

  pub async fn add_flag(&mut self, flag: i32) {
    let mut flags = self.flags.lock().await;
    *flags |= flag;
  }

  pub fn is_flag_set(flags: i32, flag_mask: i32) -> bool {
    let i = flags & flag_mask;
    i == flag_mask
  }
}

#[derive(Clone)]
pub struct ClientMessage {
  pub start_frame: Option<Frame>,
  pub next_frame: Option<Frame>,
  pub end_frame: Option<Frame>,
  pub retryable: bool,
  pub connection: Option<Connection>,
  pub cached_total_length: Option<i32>,
}

impl ClientMessage {
  pub const MESSAGE_TYPE_OFFSET: i32 = 0;
  pub const CORRELATION_ID_OFFSET: i32 = Self::MESSAGE_TYPE_OFFSET + BitsUtil::INT_SIZE_IN_BYTES;
  pub const RESPONSE_BACKUP_ACKS_OFFSET: i32 = Self::CORRELATION_ID_OFFSET + BitsUtil::LONG_SIZE_IN_BYTES;
  pub const PARTITION_ID_OFFSET: i32 = Self::CORRELATION_ID_OFFSET + BitsUtil::LONG_SIZE_IN_BYTES;
  pub const FRAGMENTATION_ID_OFFSET: i32 = 0;
  pub const DEFAULT_FLAGS: i32 = 0;
  pub const BEGIN_FRAGMENT_FLA: i32 = 1 << 15;
  pub const END_FRAGMENT_FLAG: i32 = 1 << 14;
  pub const UNFRAGMENTED_MESSAGE: i32 = Frame::BEGIN_FRAGMENT_FLAG | Self::END_FRAGMENT_FLAG;
  pub const IS_FINAL_FLAG: i32 = 1 << 13;
  pub const BEGIN_DATA_STRUCTURE_FLAG: i32 = 1 << 12;
  pub const END_DATA_STRUCTURE_FLAG: i32 = 1 << 11;
  pub const IS_NULL_FLAG: i32 = 1 << 10;
  pub const IS_EVENT_FLAG: i32 = 1 << 9;
  pub const IS_BACKUP_AWARE_FLAG: i32 = 1 << 8;
  pub const IS_BACKUP_EVENT_FLAG: i32 = 1 << 7;

  /** @internal */
  pub const SIZE_OF_FRAME_LENGTH_AND_FLAGS: i32 = BitsUtil::INT_SIZE_IN_BYTES + BitsUtil::SHORT_SIZE_IN_BYTES;

  pub async fn new(start_frame: Option<Frame>, end_frame: Option<Frame>) -> Self {
    Self {
      start_frame: start_frame.as_ref().map(|frame| frame.copy()),
      next_frame: start_frame.as_ref().map(|frame| frame.copy()),
      end_frame: end_frame.or(start_frame).map(|frame| frame.copy()),
      retryable: false,
      connection: None,
      cached_total_length: None,
    }
  }

  pub async fn create_for_encode() -> Self {
    Self::new(None, None).await
  }

  pub async fn set_correlation_id(&mut self, correlation_id: u64) {
    FixSizedTypesCodec::encode_non_negative_number_as_long(&mut *self.start_frame.as_mut().unwrap().content.lock().await, Self::CORRELATION_ID_OFFSET as usize, correlation_id).await;
  }

  pub fn merge(&mut self, fragment: ClientMessage) {
    self.end_frame.as_mut().unwrap().next = Arc::new(Mutex::new(fragment.start_frame.map(|frame| Box::new(frame.copy()))));
    self.end_frame = fragment.end_frame;
    self.cached_total_length = None;
  }

  pub async fn create_for_decode(start_frame: Frame, end_frame: Option<Frame>) -> Self {
    Self::new(Some(start_frame), end_frame).await
  }

  pub async fn next_frame(&mut self) -> Option<Frame> {
    let result = self.next_frame.as_ref().map(|frame| frame.copy());
    if result.is_some() {
      self.next_frame = if let Some(next_frame) = self.next_frame.as_ref() {
        next_frame.next.lock().await.as_ref().map(|frame| frame.copy())
      } else {
        None
      };
    }
    result
  }

  pub fn has_next_frame(&self) -> bool {
    self.next_frame.is_some()
  }

  pub fn peek_next_frame(&self) -> Option<Frame> {
    self.next_frame.as_ref().map(|frame| frame.copy())
  }

  pub fn reset_next_frame(&mut self) {
    self.next_frame = self.start_frame.as_ref().map(|frame| frame.copy());
  }

  pub async fn add_frame(&mut self, frame: Frame) {
    self.cached_total_length = None;
    *frame.next.lock().await = None;
    if self.start_frame.is_none() {
      self.start_frame = Some(frame.copy());
      self.end_frame = Some(frame.copy());
      self.next_frame = Some(frame.copy());
      return;
    }
    *self.end_frame.as_mut().unwrap().next.lock().await = Some(Box::from(frame.copy()));
    self.end_frame = Some(frame);
  }

  pub async fn get_message_type(&self) -> i32 {
    BitsUtil::read_int32(&self.start_frame.as_ref().unwrap().content.lock().await, Self::MESSAGE_TYPE_OFFSET as usize, false)
  }

  pub async fn set_message_type(&mut self, message_type: i32) {
    BitsUtil::write_int32(&mut *self.start_frame.as_mut().unwrap().content.lock().await, Self::MESSAGE_TYPE_OFFSET as usize, message_type, false);
  }

  pub async fn get_correlation_id(&self) -> u64 {
    FixSizedTypesCodec::decode_number_from_long(&*self.start_frame.as_ref().unwrap().content.lock().await, Self::CORRELATION_ID_OFFSET as usize).await
  }

  pub async fn reset_correlation(&mut self) {
    //todo: check if this is correct????
    BitsUtil::write_int32(&mut *self.start_frame.as_mut().unwrap().content.lock().await, Self::CORRELATION_ID_OFFSET as usize, -1_i32, false);
    BitsUtil::write_int32(&mut *self.start_frame.as_mut().unwrap().content.lock().await, Self::CORRELATION_ID_OFFSET as usize + BitsUtil::INT_SIZE_IN_BYTES as usize, -1_i32, false);
  }

  pub async fn get_partition_id(&self) -> i32 {
    BitsUtil::read_int32(&*self.start_frame.as_ref().unwrap().content.lock().await, Self::PARTITION_ID_OFFSET as usize, false)
  }

  pub async fn set_partition_id(&mut self, partition_id: i32) {
    BitsUtil::write_int32(&mut *self.start_frame.as_mut().unwrap().content.lock().await, Self::PARTITION_ID_OFFSET as usize, partition_id, false);
  }

  pub async fn get_number_of_backup_acks(&self) -> u8 {
    BitsUtil::read_uint8(&self.start_frame.as_ref().unwrap().content.lock().await, Self::RESPONSE_BACKUP_ACKS_OFFSET as usize)
  }

  pub async fn get_total_length(&mut self) -> i32 {
    if let Some(cached_total_length) = &self.cached_total_length {
      return *cached_total_length;
    }
    let mut total_length = 0;
    let mut current_frame = self.start_frame.as_ref().map(|frame| frame.copy());
    while let Some(frame) = current_frame {
      total_length += frame.get_length().await;
      current_frame = frame.next.lock().await.as_ref().map(|next| next.copy());
    }
    self.cached_total_length = Some(total_length);
    total_length
  }

  pub async fn get_fragmentation_id(&self) -> i64 {
    FixSizedTypesCodec::decode_long(&*self.start_frame.as_ref().unwrap().content.lock().await, Self::FRAGMENTATION_ID_OFFSET as usize).await
  }

  pub async fn drop_fragmentation_frame(&mut self) {
    let start_frame_old = std::mem::replace(&mut self.start_frame, None);
    self.start_frame = start_frame_old.as_ref().unwrap().next.lock().await.as_ref().map(|next| next.copy());
    let next_frame_old = std::mem::replace(&mut self.next_frame, None);
    self.next_frame = next_frame_old.as_ref().unwrap().next.lock().await.as_ref().map(|next| next.copy());
    self.cached_total_length = None;
  }

  pub async fn copy_with_new_correlation_id(&self) -> Self {
    let start_frame_copy = self.start_frame.as_ref().unwrap().deep_copy().await;
    let mut new_message = Self::new(Some(start_frame_copy), Some(self.end_frame.as_ref().unwrap().copy())).await;

    new_message.reset_correlation().await;
    new_message.retryable = self.retryable;
    new_message
  }

  pub async fn write_to(&self, buffer: &mut Vec<u8>, offset: Option<usize>) -> usize {
    let mut pos = offset.unwrap_or(0);
    let mut current_frame = self.start_frame.as_ref().map(|frame| frame.copy());
    while let Some(ref current_frame_value) = current_frame {
      let new_current_frame = {
        let current_frame_value_next = &mut *current_frame_value.next.lock().await;
        let current_frame_content = &*current_frame_value.content.lock().await;
        let current_frame_flags = *current_frame_value.flags.lock().await;

        let is_last_frame = current_frame_value_next.is_none();
        BitsUtil::write_int32(buffer, pos, (current_frame_content.len() + Self::SIZE_OF_FRAME_LENGTH_AND_FLAGS as usize) as i32, false);

        if is_last_frame {
          BitsUtil::write_uint16(buffer, pos + BitsUtil::INT_SIZE_IN_BYTES as usize, (current_frame_flags | Self::IS_FINAL_FLAG) as u16, false);
        } else {
          BitsUtil::write_uint16(buffer, pos + BitsUtil::INT_SIZE_IN_BYTES as usize, current_frame_flags as u16, false);
        }
        pos += Self::SIZE_OF_FRAME_LENGTH_AND_FLAGS as usize;
        buffer[pos..pos + current_frame_content.len()].copy_from_slice(&current_frame_content);
        pos += current_frame_content.len();
        current_frame_value_next.as_ref().map(|next| next.copy())
      };
      current_frame = new_current_frame;
    }
    pos
  }

  pub async fn to_buffer(&mut self) -> Vec<u8> {
    let total_length = self.get_total_length().await;
    let mut buffer = vec![0; total_length as usize];
    self.write_to(&mut buffer, None).await;
    buffer
  }
}
