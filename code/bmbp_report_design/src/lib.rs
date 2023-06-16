mod client;
mod db;
mod meta;

#[cfg(test)]
mod tests {
    use crate::{client::ModelClientFactory, db::DataBase};

    #[test]
    fn mysql_client_model() {
        let client_rs = ModelClientFactory::client("mysql".to_string());
        match client_rs {
            Ok(cli) => {
                let sql = cli.database_sql(&DataBase::new());
                println!("{:#?}", sql);
            }
            Err(e) => {
                println!("{:#?}", e);
                assert!(false);
            }
        }
    }
}
