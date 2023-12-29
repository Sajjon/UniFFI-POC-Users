uniffi::setup_scaffolding!();
#[allow(unused)]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct AccountFlags {
    flags: Vec<AccountFlag>,
}

impl AccountFlags {
    pub fn new() -> Self {
        Self { flags: Vec::new() }
    }
    pub fn add(&mut self, flag: AccountFlag) {
        self.flags.push(flag)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum Environment {
    Prod,
    Test,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum AccountFlag {
    DeletedByUser,
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct User {
    id: String,
    name: String,
    flags: AccountFlags,
}
impl User {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            flags: AccountFlags::new(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_flags(&self) -> Vec<AccountFlag> {
        self.flags.flags.clone()
    }
    pub fn add_flag_deleted_by_user(&mut self) {
        self.flags.add(AccountFlag::DeletedByUser);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct Users {
    environment: Environment,
    users: Vec<User>,
}
impl Users {
    pub fn new(environment: Environment) -> Users {
        Self {
            environment,
            users: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.users.len()
    }

    pub fn add_user(&mut self, named: &str) {
        self.users.push(User::new(named))
    }

    pub fn change_name(&mut self, user_index: usize, to: String) {
        self.users[user_index].name = to
    }

    pub fn add_flag_deleted_by_user(&mut self, user_index: usize) {
        self.users[user_index].add_flag_deleted_by_user()
    }
}

#[derive(Debug, uniffi::Object)]
pub struct Holder {
    users: RwLock<Users>,
}

impl Holder {
    fn read<T: Clone, F>(&self, access: F) -> T
    where
        F: Fn(RwLockReadGuard<'_, Users>) -> T,
    {
        access(self.users.try_read().unwrap())
    }

    fn write<F>(&self, mutate: F)
    where
        F: Fn(RwLockWriteGuard<'_, Users>),
    {
        mutate(self.users.try_write().unwrap())
    }

    fn len(&self) -> usize {
        self.read(|u| u.len())
    }
}

#[uniffi::export]
impl Holder {
    #[uniffi::constructor]
    pub fn new(environment: Environment) -> Arc<Self> {
        Arc::new(Self {
            users: RwLock::new(Users::new(environment)),
        })
    }

    pub fn add_user(&self, named: String) {
        self.write(|mut u| u.add_user(named.as_str()))
    }

    pub fn change_name_of_user(&self, at: u32, to: String) {
        self.write(|mut u| u.change_name(at as usize, to.to_owned()))
    }

    pub fn add_flag_deleted_by_user(&self, at: u32) {
        self.write(|mut u| u.add_flag_deleted_by_user(at as usize))
    }

    pub fn user_count(&self) -> u32 {
        self.len() as u32
    }
    pub fn get_users(&self) -> Vec<User> {
        self.read(|u| u.users.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{AccountFlag, Environment, Holder};

    #[test]
    fn test() {
        let holder = Holder::new(Environment::Prod);
        assert_eq!(holder.user_count(), 0);
        holder.add_user("Foo".to_string());

        assert_eq!(holder.get_users()[0].get_name(), "Foo");
        holder.change_name_of_user(0, "Bar".to_string());
        assert_eq!(holder.get_users()[0].get_name(), "Bar");

        assert_eq!(holder.get_users()[0].get_flags(), []);
        holder.add_flag_deleted_by_user(0);
        assert_eq!(
            holder.get_users()[0].get_flags(),
            [AccountFlag::DeletedByUser]
        );

        holder.add_user("Biz".to_string());
        holder.change_name_of_user(1, "Buz".to_string());
        assert_eq!(
            holder
                .get_users()
                .into_iter()
                .map(|u| u.get_name())
                .collect::<Vec<String>>(),
            ["Bar", "Buz"]
        );
    }
}
