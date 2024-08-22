use crate::auth::{SESSIONS_KEY_PREFIX, SESSION_LIFE_TIME};
use crate::models::{
    Company, NewCompany, NewRole, NewUser, NewUserCompanyRoles, NewUserRole, Role, RoleCode,
    UpdatedUserInfo, User, UserCompanyRoles, UserRole, UserType,
};
use crate::rocket_routes::CacheConnection;
use crate::schema::{companies, roles, user_company_roles, user_roles, users};
use diesel::{prelude::*, RunQueryDsl};
use rocket_db_pools::deadpool_redis::redis::RedisError;
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

pub struct UserRepository;

impl UserRepository {
    pub fn create(
        connection: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user: User = diesel::insert_into(users::table)
            .values(new_user)
            .get_result(connection)?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(connection, &role_code) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    // TODO: It is necessary to remove the possibility of creating new roles in this way
                    let name = role_code.to_string();
                    let new_role = NewRole {
                        name,
                        code: role_code,
                    };
                    let role = RoleRepository::create(connection, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(user_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(connection)?;
        }

        Ok(user)
    }

    pub fn find(connection: &mut PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(connection)
    }

    pub fn find_with_roles(connection: &mut PgConnection) -> QueryResult<Vec<(User, Vec<Role>)>> {
        let users = users::table.load(connection)?;

        let user_roles: Vec<Vec<(UserRole, Role)>> = user_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(connection)?
            .grouped_by(&users);

        let roles_by_users = user_roles
            .into_iter()
            .map(|user_roles: Vec<(UserRole, Role)>| {
                user_roles
                    .into_iter()
                    .map(|(_user_role, role)| (role))
                    .collect::<Vec<Role>>()
            });

        Ok(users.into_iter().zip(roles_by_users).collect())
    }

    pub fn find_by_email(connection: &mut PgConnection, email: &str) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(email))
            .first(connection)
    }

    pub async fn find_id_by_temporary_token(
        token: &str,
        prefix: &str,
        cache: &mut Connection<CacheConnection>,
    ) -> Result<i32, RedisError> {
        cache.get::<_, i32>(format!("{}/{}", prefix, token)).await
    }

    pub fn update_password(
        connection: &mut PgConnection,
        id: i32,
        password: &String,
    ) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set(users::password.eq(password))
            .get_result(connection)
    }

    pub fn delete(connection: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(connection)
    }

    pub fn confirm_signup(connection: &mut PgConnection, id: i32) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set(users::confirmed.eq(true))
            .get_result(connection)
    }

    pub fn update_user(
        connection: &mut PgConnection,
        id: i32,
        user_info: UpdatedUserInfo,
    ) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set(user_info)
            .get_result(connection)
    }

    pub fn set_user_type(
        connection: &mut PgConnection,
        id: i32,
        user_type: &UserType,
    ) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set(users::user_type.eq(user_type))
            .get_result(connection)
    }
}

pub struct RoleRepository;

impl RoleRepository {
    pub fn create(connection: &mut PgConnection, role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(role)
            .get_result::<Role>(connection)
    }

    pub fn find_by_code(connection: &mut PgConnection, code: &RoleCode) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(connection)
    }

    pub fn find_by_ids(connection: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .get_results(connection)
    }

    pub fn find_by_user(connection: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user).get_results(connection)?;
        let role_ids = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(connection, role_ids)
    }
}

pub struct SessionRepository;

impl SessionRepository {
    pub async fn cache_session_id(
        session_id: &String,
        user_id: i32,
        mut cache: Connection<CacheConnection>,
    ) -> Result<(), RedisError> {
        cache
            .set_ex::<_, _, ()>(
                format!("{}/{}", SESSIONS_KEY_PREFIX, session_id),
                user_id,
                SESSION_LIFE_TIME,
            )
            .await
    }

    pub async fn cache_token(
        token: &str,
        user_id: i32,
        prefix: &str,
        lifetime: usize,
        mut cache: Connection<CacheConnection>,
    ) -> Result<(), RedisError> {
        cache
            .set_ex::<_, _, ()>(format!("{}/{}", prefix, token), user_id, lifetime)
            .await
    }

    pub async fn redeem_token(
        token: &str,
        prefix: &str,
        cache: &mut Connection<CacheConnection>,
    ) -> Result<(), RedisError> {
        cache.del(format!("{}/{}", prefix, token)).await
    }
}

pub struct CompanyRepository;

impl CompanyRepository {
    pub fn create(connection: &mut PgConnection, new_company: NewCompany) -> QueryResult<Company> {
        diesel::insert_into(companies::table)
            .values(new_company)
            .get_result::<Company>(connection)
    }

    pub fn find_by_name(connection: &mut PgConnection, name: &str) -> QueryResult<Company> {
        companies::table
            .filter(companies::name.eq(name))
            .first(connection)
    }

    pub fn list(connection: &mut PgConnection) -> QueryResult<Vec<Company>> {
        let companies = companies::table.load(connection)?;
        Ok(companies)
    }

    pub fn delete(connection: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(companies::table.find(id)).execute(connection)
    }

    pub fn add_user(
        connection: &mut PgConnection,
        company: &Company,
        user: &User,
        role_codes: &Vec<RoleCode>,
    ) -> QueryResult<()> {
        for role_code in role_codes {
            if let Ok(role) = RoleRepository::find_by_code(connection, role_code) {
                let relationship = NewUserCompanyRoles {
                    user_id: user.id,
                    company_id: company.id,
                    role_id: role.id,
                };

                diesel::insert_into(user_company_roles::table)
                    .values(relationship)
                    .get_result::<UserCompanyRoles>(connection)?;
            }
        }
        Ok(())
    }
}
