#[allow(unused)]

mod u0 {
    use std::{
        cell::RefCell,
        ops::Deref,
        sync::{Arc, RwLock, RwLockReadGuard},
    };

    use uuid::Uuid;

    #[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
    pub enum Environment {
        Prod,
        Test,
    }

    #[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
    pub struct User {
        id: String,
        name: String,
    }
    impl User {
        pub fn new(name: &str) -> Self {
            Self {
                id: Uuid::new_v4().to_string(),
                name: name.to_string(),
            }
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
    }

    #[uniffi::export]
    pub fn new_users(environment: Environment) -> Users {
        Users::new(environment)
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
            self.users.try_write().unwrap().add_user(named.as_str())
        }

        pub fn user_count(&self) -> u32 {
            self.len() as u32
        }
    }
}

uniffi::include_scaffolding!("users");

#[cfg(test)]
mod test_u0 {
    use crate::u0::{Environment, Holder};

    #[test]
    fn u0() {
        let holder = Holder::new(Environment::Prod);
        assert_eq!(holder.user_count(), 0);
        holder.add_user("Foo".to_string());
        assert_eq!(holder.user_count(), 1);
    }
}
