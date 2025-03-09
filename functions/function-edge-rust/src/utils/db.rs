use anyhow::{Error, Ok, Result};
use mysql::*;
use mysql::prelude::*;
use crate::utils::models;

pub struct SqlDb{}

impl SqlDb {
    fn _get_dns(&self, cfg: &models::AppConfig) -> String {
        format!("mysql://{}:{}@{}:3306/{}", &cfg.sql_db_username, &cfg.sql_db_password, &cfg.sql_db_host, &cfg.sql_db_name)
    }
    
    pub async fn _ping_db(&self, cfg: &models::AppConfig ) -> Result<(), Error>{
        let dns = self._get_dns(cfg);
        let pool = Pool::new(dns.as_str())?;
        let mut conn = pool.get_conn()?;
        let query = format!("SELECT 1");
        let update = conn.prep(query)?;
        conn.exec_drop(&update, ())?;
        Ok(())
    }
}