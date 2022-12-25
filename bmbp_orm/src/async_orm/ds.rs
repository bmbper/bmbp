#[derive(Debug, Clone)]
pub struct BmbpDataSourcePool {
    init_size: usize,
    max_size: usize,
}

impl BmbpDataSourcePool {
    pub fn init_size(&self) -> &usize {
        &self.init_size
    }
    pub fn set_init_size(&mut self, init_size: usize) -> &mut Self {
        self.init_size = init_size;
        self
    }
    pub fn max_size(&self) -> &usize {
        &self.max_size
    }
    pub fn set_max_size(&mut self, max_size: usize) -> &mut Self {
        self.max_size = max_size;
        self
    }
}

impl Default for BmbpDataSourcePool {
    fn default() -> Self {
        Self {
            init_size: 5,
            max_size: 1000,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BmbpDataSource {
    driver: String,
    host: String,
    port: usize,
    database: String,
    schema: String,
    user: String,
    password: String,
    ignore_case: bool,
    pool_config: BmbpDataSourcePool,
}
impl BmbpDataSource {
    pub fn new() -> Self {
        BmbpDataSource::default()
    }
    pub fn driver(&self) -> &String {
        &self.driver
    }
    pub fn set_driver(&mut self, dirver: String) -> &mut Self {
        self.driver = dirver;
        self
    }

    pub fn host(&self) -> &String {
        &self.host
    }
    pub fn set_host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }

    pub fn port(&self) -> &usize {
        &self.port
    }
    pub fn set_port(&mut self, port: usize) -> &mut Self {
        self.port = port;
        self
    }

    pub fn database(&self) -> &String {
        &self.database
    }

    pub fn set_database(&mut self, database: String) -> &mut Self {
        self.database = database;
        self
    }

    pub fn schema(&self) -> &String {
        &self.schema
    }

    pub fn set_schema(&mut self, schema: String) -> &mut Self {
        self.schema = schema;
        self
    }

    pub fn user(&self) -> &String {
        &self.user
    }

    pub fn set_user(&mut self, user: String) -> &mut Self {
        self.user = user;
        self
    }
    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn set_password(&mut self, password: String) -> &mut Self {
        self.password = password;
        self
    }

    pub fn ignore_case(&self) -> &bool {
        &self.ignore_case
    }

    pub fn set_ignore_case(&mut self, ignore_case: bool) -> &mut Self {
        self.ignore_case = ignore_case;
        self
    }

    pub fn pool_config(&self) -> &BmbpDataSourcePool {
        &self.pool_config
    }

    pub fn set_pool_config(&mut self, pool_config: BmbpDataSourcePool) -> &mut Self {
        self.pool_config = pool_config;
        self
    }
}
