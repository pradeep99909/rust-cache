#[derive()]
pub enum Type {
    READ,
    WRITE
}

impl Type{
    pub fn to_int(opr: &str) -> Type {
        match opr {
            "READ" => Type::READ,
            "WRITE" => Type::WRITE,
            _ => panic!("Wrong Opeartion!")
        }
    }
}