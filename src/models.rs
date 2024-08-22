use std::{fmt, io::Write, str::FromStr};

use crate::schema::{companies, roles, user_company_roles, user_roles, users};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::{AsChangeset, Associations, Identifiable},
    serialize::{IsNull, Output, ToSql},
    sql_types::Text,
    Insertable, Queryable,
};
use serde::Serialize;

#[derive(Queryable, Debug, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub confirmed: bool,
    pub updated_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub user_type: UserType,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Debug)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=user_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(AsChangeset)]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name=users)]
pub struct UpdatedUserInfo {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Queryable, Debug, Identifiable, Serialize)]
#[diesel(table_name = companies)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = companies)]
pub struct NewCompany {
    pub name: String,
    pub email: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(table_name = user_company_roles)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Company))]
#[diesel(belongs_to(Role))]
pub struct UserCompanyRoles {
    pub id: i32,
    pub user_id: i32,
    pub company_id: i32,
    pub role_id: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=user_company_roles)]
pub struct NewUserCompanyRoles {
    pub user_id: i32,
    pub company_id: i32,
    pub role_id: i32,
}

#[derive(AsExpression, FromSqlRow, Debug)]
#[diesel(sql_type=Text)]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl fmt::Display for RoleCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoleCode::Admin => write!(f, "admin"),
            RoleCode::Editor => write!(f, "editor"),
            RoleCode::Viewer => write!(f, "viewer"),
        }
    }
}

impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"editor" => Ok(RoleCode::Editor),
            b"viewer" => Ok(RoleCode::Viewer),
            _ => Ok(RoleCode::Viewer),
        }
    }
}

impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?,
        };
        Ok(IsNull::No)
    }
}

#[derive(AsExpression, FromSqlRow, Debug, PartialEq)]
#[diesel(sql_type=Text)]
pub enum UserType {
    Regular,
    Enterprise,
}

impl fmt::Display for UserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserType::Regular => write!(f, "regular"),
            UserType::Enterprise => write!(f, "enterprise"),
        }
    }
}

impl FromStr for UserType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "regular" => Ok(UserType::Regular),
            "enterprise" => Ok(UserType::Enterprise),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for UserType {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"regular" => Ok(UserType::Regular),
            b"enterprise" => Ok(UserType::Enterprise),
            _ => Ok(UserType::Regular),
        }
    }
}

impl ToSql<Text, Pg> for UserType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            UserType::Regular => out.write_all(b"regular")?,
            UserType::Enterprise => out.write_all(b"enterprise")?,
        };
        Ok(IsNull::No)
    }
}
