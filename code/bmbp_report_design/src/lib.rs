mod client;
mod db;
mod meta;

#[cfg(test)]
mod tests {
    use crate::client::ModelClientFactory;

    #[test]
    fn mysql_client_model() {
        let client_rs = ModelClientFactory::client("mysql".to_string());
        match client_rs {
            Ok(_cli) => {}
            Err(e) => {
                println!("{:#?}", e);
                assert!(false);
            }
        }
    }
}
