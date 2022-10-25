# ``serde``

#### Example: custom range type
```Rust
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use sqlx::postgres::types::PgRange;


#[derive(Debug, Clone, Copy, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "check_range"))]
pub struct SoRange<T: std::cmp::PartialOrd> {
    pub min: T,
    pub max: T
}

impl<T: std::cmp::PartialOrd> SoRange<T>  {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

fn check_range<T: std::cmp::PartialOrd>(range: &SoRange<T>) -> Result<(), ValidationError> {
    if &range.max < &range.min {
        Err(ValidationError::new(
            "'max' can not be less then 'min'.",
        ))
    }
    else {
        Ok(())
    }
}

impl<T: std::cmp::PartialOrd, U> From<&SoRange<T>> for PgRange<U> 
    where 
        U: From<T>,
        T: Clone {
    fn from(r: &SoRange<T>) -> Self {
        PgRange {
            start: std::ops::Bound::Included(r.min.clone().into()),
            end: std::ops::Bound::Included(r.max.clone().into()),
        }
    }
}
```