use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub role: Role,
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin = 0,
    #[default]
    User = 1,
    // It means the user is not signed in web page.
    Guest = 10,
}

impl TryFrom<i32> for Role {
    type Error = String;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Role::Admin),
            1 => Ok(Role::User),
            10 => Ok(Role::Guest),
            _ => Err(format!("Invalid Role: {}", value)),
        }
    }
}

impl Into<i32> for Role {
    fn into(self) -> i32 {
        self as i32
    }
}

#[cfg(feature = "server")]
impl sqlx::Type<sqlx::Postgres> for Role {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "server")]
impl sqlx::Encode<'_, sqlx::Postgres> for Role {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> std::result::Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let value: i32 = (*self).clone().into();
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&value, buf)
    }
}

#[cfg(feature = "server")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Role {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let int_value: i32 = <i32 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Role::try_from(int_value)
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)).into())
    }
}
