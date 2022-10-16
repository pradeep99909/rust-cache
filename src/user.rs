

pub mod UserMod{
    use crate::model;
    pub struct User {
        pub users: Vec<model::Struct::UserModel>,
        pub userLength: i128
    }
    impl User {

        pub fn init(&mut self) {
            self.users = Vec::new();
            self.userLength = 0;
        }
        pub fn create(&mut self) -> model::Struct::UserModel { 
            let user_len = self.userLength + 1;
            self.userLength = user_len;
            let user: model::Struct::UserModel = model::Struct::UserModel {
                id: self.userLength,
                name: String::from("ABCD")
            };
            self.add_user(&user);
            return user;
        }
        fn add_user(&mut self, user: &model::Struct::UserModel) {
            let new_user = user;
            self.users.push(model::Struct::UserModel{
                id: new_user.id,
                name: String::from(&new_user.name)
            });
        }

        pub fn read(&self) -> &model::Struct::UserModel {
            return &self.users[0];
        }
    }
}