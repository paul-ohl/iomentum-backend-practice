use config::{builder::DefaultState, ConfigBuilder, Environment};
use serde::Deserialize;

pub struct Cfg {
    pub db_url: String,
}

#[derive(Deserialize)]
struct CfgBuilder {
    db_user: String,
    db_password: String,
    db_port: String,
}

impl From<CfgBuilder> for Cfg {
    fn from(cfg: CfgBuilder) -> Self {
        Cfg::new(cfg.db_user, cfg.db_password, cfg.db_port)
    }
}

impl Cfg {
    fn new(db_user: String, db_password: String, db_port: String) -> Self {
        Cfg {
            db_url: format!(
                "postgresql://{db_user}:{db_password}@localhost:{db_port}/backend_practice"
            ),
        }
    }

    pub fn init() -> Self {
        dotenv::dotenv().ok();
        let cfg = ConfigBuilder::<DefaultState>::default().add_source(Environment::default());

        let cfg_builder: CfgBuilder = cfg
            .build()
            .expect("cannot build config")
            .try_deserialize()
            .expect("cannot convert config");
        cfg_builder.into()
    }
}
