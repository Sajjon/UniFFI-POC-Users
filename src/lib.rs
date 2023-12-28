#[allow(unused)]

mod u0 {
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
        pub fn new<I>(environment: Environment, users: I) -> Self
        where
            I: IntoIterator<Item = User>,
        {
            Self {
                environment,
                users: users.into_iter().collect(),
            }
        }
        pub fn empty_prod() -> Self {
            Self::new(Environment::Prod, [])
        }
        pub fn new_user(&mut self, name: &str) {
            self.users.push(User::new(name))
        }
        pub fn len(&self) -> usize {
            self.users.len()
        }
    }
}

uniffi::include_scaffolding!("users");

#[cfg(test)]
mod test_u0 {
    use crate::u0::Users;

    #[test]
    fn u0() {
        let mut users = Users::empty_prod();
        assert_eq!(users.len(), 0);
        users.new_user("Foo");
        assert_eq!(users.len(), 1);
    }
}
