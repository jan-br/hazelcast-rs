use crate::connection::address::Address;
use crate::serialization::serializable::{IdentifiedDataSerializable, IdentifiedDataSerializableSerialization};

pub const CLUSTER_DATA_FACTORY_ID: i32 = 0;
pub const CLUSTER_DATA_ADDRESS_CLASS_ID: i32 = 1;

pub fn cluster_data_factory(class_id: i32) -> Box<dyn IdentifiedDataSerializable> {
    if class_id == CLUSTER_DATA_ADDRESS_CLASS_ID {
        Box::new(Address::new(None, None))
    } else {
        todo!()
    }
}