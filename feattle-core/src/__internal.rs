//! Internal types and re-exports used by the macros

pub use crate::json_reading::FromJsonError;
pub use crate::persist::{CurrentValue, Persist};
pub use crate::{FeattleDefinition, Feattles, FeattlesPrivate};
pub use parking_lot::{MappedRwLockReadGuard, RwLockReadGuard, RwLockWriteGuard};

use crate::last_reload::LastReload;
use crate::persist::CurrentValues;
use crate::FeattleValue;
use parking_lot::RwLock;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::{fmt, mem};

/// The main implementation of this crate. The struct generated by the macro [`feattles!`] is just
/// a new-type over this struct.
pub struct FeattlesImpl<FS> {
    pub persistence: Arc<dyn Persist>,
    pub inner_feattles: RwLock<InnerFeattles<FS>>,
}

/// The main content of a `Feattles` instance, protected behind a lock
#[derive(Debug, Clone)]
pub struct InnerFeattles<FS> {
    pub last_reload: LastReload,
    pub current_values: Option<CurrentValues>,
    pub feattles_struct: FS,
}

/// The generic representation of each feattle inside the feattles struct
#[derive(Debug, Clone)]
pub struct Feattle<T> {
    key: &'static str,
    description: &'static str,
    value: T,
    default: T,
    current_value: Option<CurrentValue>,
}

#[derive(Copy, Clone, Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matching variant not found")
    }
}

impl Error for ParseError {}

/// The auto-generated internal struct will implement this trait
pub trait FeattlesStruct: 'static {
    /// Try to update the given key, returning the previous value, if any.
    fn try_update(
        &mut self,
        key: &str,
        value: Option<CurrentValue>,
    ) -> Result<Option<CurrentValue>, FromJsonError>;
}

impl<FS> FeattlesImpl<FS> {
    pub fn new(persistence: Arc<dyn Persist>, feattles_struct: FS) -> Self {
        FeattlesImpl {
            persistence,
            inner_feattles: RwLock::new(InnerFeattles {
                last_reload: LastReload::Never,
                current_values: None,
                feattles_struct,
            }),
        }
    }
}

impl<T: Clone + FeattleValue> Feattle<T> {
    pub fn new(key: &'static str, description: &'static str, default: T) -> Self {
        Feattle {
            key,
            description,
            value: default.clone(),
            default,
            current_value: None,
        }
    }

    pub fn definition(&self) -> FeattleDefinition {
        FeattleDefinition {
            key: self.key,
            description: self.description.to_owned(),
            format: T::serialized_format(),
            value: self.value.as_json(),
            value_overview: self.value.overview(),
            default: self.default.as_json(),
            modified_at: self.current_value.as_ref().map(|v| v.modified_at),
            modified_by: self.current_value.as_ref().map(|v| v.modified_by.clone()),
        }
    }

    /// Try to update this value, returning the previous value, if any.
    pub fn try_update(
        &mut self,
        value: Option<CurrentValue>,
    ) -> Result<Option<CurrentValue>, FromJsonError> {
        // Note: we must call `try_from_json` to fail **before** updating anything
        self.value = match &value {
            None => self.default.clone(),
            Some(value) => FeattleValue::try_from_json(&value.value)?,
        };
        Ok(mem::replace(&mut self.current_value, value))
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<FS: Debug> Debug for FeattlesImpl<FS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FeattlesImpl")
            .field("persistence", &"Arc<dyn Persist>")
            .field("inner_feattles", &self.inner_feattles)
            .finish()
    }
}
