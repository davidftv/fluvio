mod spec;
mod status;
mod policy;
mod replica;
pub mod store;

pub use self::spec::*;
pub use self::status::*;
pub use dataplane::ReplicaKey;
pub use self::policy::*;
pub use self::replica::*;

#[cfg(feature = "k8")]
mod k8;
#[cfg(feature = "k8")]
pub use k8::*;

mod metadata {

    use crate::partition::ReplicaKey;
    use crate::core::*;
    use crate::topic::TopicSpec;
    use super::*;

    impl Spec for PartitionSpec {
        const LABEL: &'static str = "Partition";
        type IndexKey = ReplicaKey;
        type Status = PartitionStatus;
        type Owner = TopicSpec;
    }

    impl Status for PartitionStatus {}

    #[cfg(feature = "k8")]
    mod extended {

        use crate::store::k8::K8ExtendedSpec;
        use crate::store::k8::K8ConvertError;
        use crate::store::k8::K8MetaItem;
        use crate::store::MetadataStoreObject;
        use crate::k8::metadata::K8Obj;
        use crate::store::k8::default_convert_from_k8;

        use super::metadata::PartitionSpec;

        impl K8ExtendedSpec for PartitionSpec {
            type K8Spec = Self;
            type K8Status = Self::Status;

            fn convert_from_k8(
                k8_obj: K8Obj<Self::K8Spec>,
            ) -> Result<MetadataStoreObject<Self, K8MetaItem>, K8ConvertError<Self::K8Spec>>
            {
                default_convert_from_k8(k8_obj)
            }
        }
    }
}
