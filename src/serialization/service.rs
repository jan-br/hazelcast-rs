use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};

use crate::config::serialization::{SerializationConfig};
use crate::core::rest_value::{rest_value_factory, REST_VALUE_FACTORY_ID};
use crate::proxy::topic::reliable_topic_message::{
  reliable_topic_message_factory, RELIABLE_TOPIC_MESSAGE_FACTORY_ID,
};
use crate::serialization::aggregation::aggregator::aggregator_factory;
use crate::serialization::aggregation::aggregator_constants::AGGREGATOR_FACTORY_ID;
use crate::serialization::cluster_data_factory::{cluster_data_factory, CLUSTER_DATA_FACTORY_ID};
use crate::serialization::compact::compact_stream_serializer::CompactStreamSerializer;
use crate::serialization::data::object_data_input::ObjectDataInput;
use crate::serialization::data::object_data_output::ObjectDataOutput;
use crate::serialization::data::{DataInput, DataOutput};
use crate::serialization::default_predicates::{predicate_factory, PREDICATE_FACTORY_ID};
use crate::serialization::default_serializer::IdentifiedDataSerializableSerializer;
use crate::serialization::heap_data::HeapData;
use crate::serialization::portable::serializer::PortableSerializer;
use crate::serialization::schema_service::SchemaService;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization, Serializable};
use crate::serialization::serializer::default::array_list_serializer::ArrayListSerializer;
use crate::serialization::serializer::default::big_decimal_serializer::BigDecimalSerializer;
use crate::serialization::serializer::default::big_int_serializer::BigIntSerializer;
use crate::serialization::serializer::default::boolean_array_serializer::BooleanArraySerializer;
use crate::serialization::serializer::default::boolean_serializer::BooleanSerializer;
use crate::serialization::serializer::default::byte_array_serializer::ByteArraySerializer;
use crate::serialization::serializer::default::byte_serializer::ByteSerializer;
use crate::serialization::serializer::default::char_array_serializer::CharArraySerializer;
use crate::serialization::serializer::default::char_serializer::CharSerializer;
use crate::serialization::serializer::default::date_serializer::DateSerializer;
use crate::serialization::serializer::default::double_array_serializer::DoubleArraySerializer;
use crate::serialization::serializer::default::double_serializer::DoubleSerializer;
use crate::serialization::serializer::default::float_array_serializer::FloatArraySerializer;
use crate::serialization::serializer::default::float_serializer::FloatSerializer;
use crate::serialization::serializer::default::integer_array_serializer::IntegerArraySerializer;
use crate::serialization::serializer::default::integer_serializer::IntegerSerializer;
use crate::serialization::serializer::default::java_array_serializer::JavaArraySerializer;
use crate::serialization::serializer::default::java_class_serializer::JavaClassSerializer;
use crate::serialization::serializer::default::linked_list_serializer::LinkedListSerializer;
use crate::serialization::serializer::default::local_date_serializer::LocalDateSerializer;
use crate::serialization::serializer::default::local_date_time_serializer::LocalDateTimeSerializer;
use crate::serialization::serializer::default::local_time_serializer::LocalTimeSerializer;
use crate::serialization::serializer::default::long_array_serializer::LongArraySerializer;
use crate::serialization::serializer::default::long_serializer::LongSerializer;
use crate::serialization::serializer::default::null_serializer::NullSerializer;
use crate::serialization::serializer::default::offset_date_time_serializer::OffsetDateTimeSerializer;
use crate::serialization::serializer::default::short_array_serializer::ShortArraySerializer;
use crate::serialization::serializer::default::short_serializer::ShortSerializer;
use crate::serialization::serializer::default::string_array_serializer::StringArraySerializer;
use crate::serialization::serializer::default::string_serializer::StringSerializer;
use crate::serialization::serializer::default::uuid_serializer::UuidSerializer;
use crate::serialization::serializer::Serializer;
use crate::ClientConfig;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::intrinsics::transmute;
use std::sync::Arc;
use num_bigint::BigInt;
use uuid::Uuid;
use crate::core::big_decimal::BigDecimal;
use crate::serialization::generic_record::compact_generic_record::CompactGenericRecord;
use crate::serialization::generic_record::generic_record::GenericRecord;
use crate::serialization::portable::PortableSerializable;
use crate::serialization::serializer::default::json_serializer::JsonSerializer;

pub struct SerializationServiceV1 {
  pub registry: HashMap<i32, Arc<dyn Serializer<Box<dyn Any>>>>,
  pub serializer_name_to_id: HashMap<String, i32>,
  pub compact_stream_serializer: Arc<CompactStreamSerializer>,
  pub portable_serializer: Arc<PortableSerializer>,
  pub identified_serializer: Arc<IdentifiedDataSerializableSerializer>,
  pub serialization_config: SerializationConfig,
}

impl SerializationServiceV1 {
  pub const DATA_OFFSET: usize = 8;

  pub fn new(
    serialization_config: SerializationConfig,
    schema_service: Arc<SchemaService>,
  ) -> Self {
    let mut result = Self {
      registry: HashMap::new(),
      serializer_name_to_id: HashMap::new(),
      compact_stream_serializer: Arc::new(CompactStreamSerializer::new(schema_service)),
      portable_serializer: Arc::new(PortableSerializer::new(serialization_config.clone())),
      identified_serializer: Arc::new(Self::create_identified_serializer(&serialization_config)),
      serialization_config,
    };
    result.init();
    result
  }

  pub fn init(&mut self) {
    self.register_default_serializers();
    self.register_custom_serializers();
    self.register_compact_serializers();
    self.register_global_serializer();
  }

  pub fn register_global_serializer(&mut self) {
    //todo implement global serializer
  }

  pub fn register_compact_serializers(&mut self) {
    //todo implement compact serializers
  }

  pub fn register_custom_serializers(&mut self) {
    //todo implement custom serializers
  }

  pub fn register_default_serializers(&mut self) {
    self.register_serializer("string", Arc::new(StringSerializer::default()));
    self.register_serializer("double", Arc::new(DoubleSerializer::default()));
    self.register_serializer("byte", Arc::new(ByteSerializer::default()));
    self.register_serializer("boolean", Arc::new(BooleanSerializer::default()));
    self.register_serializer("null", Arc::new(NullSerializer::default()));
    self.register_serializer("short", Arc::new(ShortSerializer::default()));
    self.register_serializer("integer", Arc::new(IntegerSerializer::default()));
    self.register_serializer("long", Arc::new(LongSerializer::default()));
    self.register_serializer("float", Arc::new(FloatSerializer::default()));
    self.register_serializer("char", Arc::new(CharSerializer::default()));
    self.register_serializer("date", Arc::new(DateSerializer::default()));
    self.register_serializer("localDate", Arc::new(LocalDateSerializer::default()));
    self.register_serializer("localTime", Arc::new(LocalTimeSerializer::default()));
    self.register_serializer("localDateTime", Arc::new(LocalDateTimeSerializer::default()));
    self.register_serializer("offsetDateTime", Arc::new(OffsetDateTimeSerializer::default()));
    self.register_serializer("byteArray", Arc::new(ByteArraySerializer::default()));
    self.register_serializer("charArray", Arc::new(CharArraySerializer::default()));
    self.register_serializer("booleanArray", Arc::new(BooleanArraySerializer::default()));
    self.register_serializer("shortArray", Arc::new(ShortArraySerializer::default()));
    self.register_serializer("integerArray", Arc::new(IntegerArraySerializer::default()));
    self.register_serializer("longArray", Arc::new(LongArraySerializer::default()));
    self.register_serializer("doubleArray", Arc::new(DoubleArraySerializer::default()));
    self.register_serializer("stringArray", Arc::new(StringArraySerializer::default()));
    self.register_serializer("javaClass", Arc::new(JavaClassSerializer::default()));
    self.register_serializer("floatArray", Arc::new(FloatArraySerializer::default()));
    self.register_serializer("arrayList", Arc::new(ArrayListSerializer::default()));

    self.register_serializer("linkedList", Arc::new(LinkedListSerializer::default()));
    self.register_serializer("uuid", Arc::new(UuidSerializer::default()));
    self.register_serializer("bigDecimal", Arc::new(BigDecimalSerializer::default()));
    self.register_serializer("bigint", Arc::new(BigIntSerializer::default()));
    self.register_serializer("javaArray", Arc::new(JavaArraySerializer::default()));
    self.register_serializer("!compact", self.compact_stream_serializer.clone());
    self.register_serializer("identified", self.identified_serializer.clone());
    self.register_serializer("!portable", self.portable_serializer.clone());
    //todo: implement lazy serializer
    self.register_serializer("!json", Arc::new(JsonSerializer::default()));
  }

  pub fn register_serializer(
    &mut self,
    name: impl ToString,
    serializer: Arc<dyn Serializer<Box<impl Any + ?Sized>>>,
  ) {
    let name = name.to_string();
    if self.serializer_name_to_id.contains_key(&name) {
      todo!()
    }
    if self.registry.contains_key(&serializer.id()) {
      todo!()
    }
    self.serializer_name_to_id.insert(name, serializer.id());
    self.registry
      .insert(serializer.id(), unsafe { transmute(serializer) });
  }

  pub fn create_identified_serializer(
    serialization_config: &SerializationConfig,
  ) -> IdentifiedDataSerializableSerializer {
    let mut factories = HashMap::new();
    serialization_config
      .data_serializable_factories
      .iter()
      .for_each(|(key, value)| {
        factories.insert(key.clone(), value.clone());
      });
    factories.insert(PREDICATE_FACTORY_ID, Arc::new(predicate_factory));
    factories.insert(
      RELIABLE_TOPIC_MESSAGE_FACTORY_ID,
      Arc::new(reliable_topic_message_factory),
    );
    factories.insert(CLUSTER_DATA_FACTORY_ID, Arc::new(cluster_data_factory));
    factories.insert(AGGREGATOR_FACTORY_ID, Arc::new(aggregator_factory));
    factories.insert(REST_VALUE_FACTORY_ID, Arc::new(rest_value_factory));
    IdentifiedDataSerializableSerializer::new(factories)
  }

  pub fn read_object(&self, input: &mut ObjectDataInput) -> Box<dyn Any> {
    let serializer_id = input.read_int();
    let serializer: Box<dyn Serializer<Box<dyn Any>>> =
      unsafe { std::mem::transmute(self.find_serializer_by_id(serializer_id)) };
    serializer.read(input)
  }

  pub fn find_serializer_by_id(&self, id: i32) -> Arc<dyn Serializer<Box<dyn Any>>> {
    self.registry.get(&id).unwrap().clone()
  }

  pub fn is_data<T: 'static>(&self, object: &T) -> bool {
    object.type_id() == TypeId::of::<HeapData>()
  }

  fn calculate_default_partition_strategy(&self) -> i32 {
    //todo: Implement
    0
  }

  pub fn to_data<T: Serializable + 'static>(self: &Arc<Self>, object: Box<T>) -> HeapData {
    if self.is_data(object.as_ref()) {
      return unsafe { transmute::<_, &HeapData>(object) }.clone();
    }
    let mut data_output = ObjectDataOutput::new(self.serialization_config.is_big_endian, self.clone());
    let serializer = object.get_serializer(self);
    //todo: check if object is partition aware
    data_output.write_int_be(self.calculate_default_partition_strategy());
    data_output.write_int_be(serializer.id());
    serializer.write(&mut data_output, object);
    HeapData::new(data_output.to_buffer())
  }

  pub async fn to_object<T: 'static>(self: &Arc<Self>, data: HeapData) -> Box<T> {
    let serializer: Box<Arc<dyn Serializer<Box<T>>>> = unsafe { transmute(Box::new(self.find_serializer_by_id(data.get_type()))) };
    let mut data_input = ObjectDataInput::new(data.to_buffer(), Self::DATA_OFFSET, self.clone(), self.serialization_config.is_big_endian);
    serializer.read(&mut data_input)
  }

  pub fn write_object<T>(&self, out: &mut ObjectDataOutput, obj: &T) {
    todo!()
  }
}

