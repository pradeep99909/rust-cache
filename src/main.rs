mod user;
mod model;
use user::UserMod::User;
fn main() {
    let mut user = User{
        users: Vec::new(),
        userLength: 0
    };
    
    let user_created = user.create();
    println!("{:?}",user_created);
    let user = user.read();
    println!("{:?}",user);
}
