use std::cmp::{max, min};
use byteorder::ByteOrder;
use crate::protocol::client_message::{ClientMessage, Frame};
use crate::util::bits_util::BitsUtil;

pub struct ClientMessageReader {
  chunks: Vec<Vec<u8>>,
  chunks_total_size: usize,
  frame_size: usize,
  flags: i32,
  client_message: Option<ClientMessage>,
}

impl ClientMessageReader {
  pub fn new() -> Self {
    ClientMessageReader {
      chunks: Vec::new(),
      chunks_total_size: 0,
      frame_size: 0,
      flags: 0,
      client_message: None,
    }
  }

  pub fn append(&mut self, data: Vec<u8>) {
    self.chunks_total_size += data.len();
    self.chunks.push(data);
  }

  pub async fn read(&mut self) -> Option<ClientMessage> {
    loop {
      if self.read_frame().await {
        if self.client_message.as_ref().unwrap().end_frame.as_ref().unwrap().is_final_frame().await {
          let message = std::mem::replace(&mut self.client_message, None).unwrap();
          self.reset();
          return Some(message);
        }
      } else {
        return None;
      }
    }
  }

  pub async fn read_frame(&mut self) -> bool {
    if self.chunks_total_size < ClientMessage::SIZE_OF_FRAME_LENGTH_AND_FLAGS as usize {
      return false;
    }
    if self.frame_size == 0 {
      self.read_frame_size_and_flags();
    }
    if self.chunks_total_size < self.frame_size {
      return false;
    }

    let mut buf = if self.chunks.len() == 1 {
      self.chunks[0].clone()
    } else {
      let mut result = vec![0_u8; self.chunks_total_size];
      self.chunks.iter().for_each(|chunk| {
        result.extend_from_slice(chunk);
      });
      result[..self.chunks_total_size].to_vec()
    };

    if self.chunks_total_size > self.frame_size {
      if self.chunks.len() == 1 {
        self.chunks[0] = buf[self.frame_size..].to_vec();
      } else {
        self.chunks = [buf[self.frame_size..].to_vec()].to_vec();
      }
      buf = buf[ClientMessage::SIZE_OF_FRAME_LENGTH_AND_FLAGS as usize..max(ClientMessage::SIZE_OF_FRAME_LENGTH_AND_FLAGS as usize, self.frame_size)].to_vec();
    } else {
      self.chunks = vec![];
      buf = buf[min(ClientMessage::SIZE_OF_FRAME_LENGTH_AND_FLAGS as usize, buf.len() - 1) as usize..].to_vec();
    }
    self.chunks_total_size -= self.frame_size;
    self.frame_size = 0;

    let frame = Frame::new(buf, self.flags);
    if let Some(ref mut client_message) = self.client_message {
      client_message.add_frame(frame).await;
    } else {
      self.client_message = Some(ClientMessage::create_for_decode(frame, None).await);
    }
    true
  }

  pub fn reset(&mut self) {
    self.client_message = None;
  }

  pub fn read_frame_size_and_flags(&mut self) {
    if self.chunks[0].len() >= ClientMessage::SIZE_OF_FRAME_LENGTH_AND_FLAGS as usize {
      self.frame_size = byteorder::LittleEndian::read_i32(&self.chunks[0][0..BitsUtil::INT_SIZE_IN_BYTES as usize]) as usize;
      self.flags = byteorder::LittleEndian::read_u16(&self.chunks[0][BitsUtil::INT_SIZE_IN_BYTES as usize..BitsUtil::INT_SIZE_IN_BYTES as usize + BitsUtil::SHORT_SIZE_IN_BYTES as usize]) as i32;
      return;
    }
    let mut read_chunk_size = 0;
    for i in 0..self.chunks.len() {
      read_chunk_size += self.chunks[i].len();
      if read_chunk_size >= ClientMessage::SIZE_OF_FRAME_LENGTH_AND_FLAGS as usize {
        let mut merged = vec![0_u8; read_chunk_size];
        self.chunks[0..i as usize + 1].iter().for_each(|chunk| {
          merged.extend_from_slice(chunk);
        });
        merged = merged[0..read_chunk_size].to_vec();
        self.frame_size = byteorder::LittleEndian::read_i32(&merged[0..BitsUtil::INT_SIZE_IN_BYTES as usize]) as usize;
        self.flags = byteorder::LittleEndian::read_u16(&merged[BitsUtil::INT_SIZE_IN_BYTES as usize..BitsUtil::INT_SIZE_IN_BYTES as usize + BitsUtil::SHORT_SIZE_IN_BYTES as usize]) as i32;
        return;
      }
    }
    todo!()
  }
}