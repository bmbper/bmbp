use uuid::Uuid;

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn uuid_upper() -> String {
    Uuid::new_v4().to_string().to_uppercase()
}

pub fn simple_uuid() -> String {
    let uid = Uuid::new_v4().to_string().to_uppercase().replace("-", "");
    uid.replace("-", "")
}

pub fn simple_uuid_upper() -> String {
    let uid = Uuid::new_v4().to_string();
    uid.replace("-", "").to_uppercase()
}

#[cfg(test)]
mod tests {
    use crate::id::{simple_uuid, simple_uuid_upper, uuid, uuid_upper};

    #[test]
    fn test_uuid() {
        println!("{}", uuid());
        println!("{}", uuid_upper());
        println!("{}", simple_uuid());
        println!("{}", simple_uuid_upper());
    }
}
