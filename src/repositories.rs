use crate::auth::SESSION_LIFE_TIME;
use crate::models::{NewRole, NewUser, NewUserRole, Role, RoleCode, User, UserRole};
use crate::rocket_routes::CacheConnection;
use crate::schema::{roles, user_roles, users};
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

    pub fn find_by_email(connection: &mut PgConnection, email: &String) -> QueryResult<User> {
        users::table
            .filter(users::email.eq(email))
            .first(connection)
    }

    pub fn delete(connection: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(user_roles::table.filter(user_roles::user_id.eq(id))).execute(connection)?;
        diesel::delete(users::table.find(id)).execute(connection)
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
                format!("sessions/{}", session_id),
                user_id,
                SESSION_LIFE_TIME,
            )
            .await
    }
}
