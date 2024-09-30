use config::{builder::DefaultState, ConfigBuilder, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Cfg {
    pub db_user: String,
    pub db_password: String,
    pub db_port: String,
    pub db_name: String,
}

impl Cfg {
    pub fn db_url(&self) -> String {
        format!("{}/{}", self.without_db_name(), self.db_name)
    }

    pub fn without_db_name(&self) -> String {
        format!(
            "postgresql://{}:{}@localhost:{}",
            self.db_user, self.db_password, self.db_port
        )
    }

    pub fn init() -> Self {
        dotenv::dotenv().ok();
        let cfg = ConfigBuilder::<DefaultState>::default().add_source(Environment::default());

        cfg.build()
            .expect("cannot build config")
            .try_deserialize()
            .expect("cannot convert config")
    }
}
