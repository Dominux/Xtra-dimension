use std::{env, str::FromStr};

use super::errors::{PentaractError, PentaractResult};

#[derive(Debug, Clone)]
pub struct Config {
    pub db_uri: String,
    pub port: u16,
    pub workers: u16,
    pub channel_capacity: u16,
    pub superuser_name: String,
    pub superuser_pass: String,

    pub access_token_expire_in_secs: u32,
    pub refresh_token_expire_in_days: u16,
    pub secret_key: String,

    pub telegram_api_base_url: String,
}

impl Config {
    pub fn new() -> PentaractResult<Self> {
        let db_uri = {
            let db_user: String = Self::get_env_var("DATABASE_USER")?;
            let db_password: String = Self::get_env_var("DATABASE_PASSWORD")?;
            let db_name: String = Self::get_env_var("DATABASE_NAME")?;
            let db_host: String = Self::get_env_var("DATABASE_HOST")?;
            let db_port: String = Self::get_env_var("DATABASE_PORT")?;

            format!("postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}")
        };
        let port = Self::get_env_var("PORT")?;
        let workers = Self::get_env_var("WORKERS")?;
        let channel_capacity = Self::get_env_var("CHANNEL_CAPACITY")?;
        let superuser_name = Self::get_env_var("SUPERUSER_NAME")?;
        let superuser_pass = Self::get_env_var("SUPERUSER_PASS")?;
        let access_token_expire_in_secs = Self::get_env_var("ACCESS_TOKEN_EXPIRE_IN_SECS")?;
        let refresh_token_expire_in_days = Self::get_env_var("REFRESH_TOKEN_EXPIRE_IN_DAYS")?;
        let secret_key = Self::get_env_var("SECRET_KEY")?;
        let telegram_api_base_url = Self::get_env_var("TELEGRAM_API_BASE_URL")?;

        Ok(Self {
            db_uri,
            port,
            workers,
            channel_capacity,
            superuser_name,
            superuser_pass,
            access_token_expire_in_secs,
            refresh_token_expire_in_days,
            secret_key,
            telegram_api_base_url,
        })
    }

    #[inline]
    fn get_env_var<T: FromStr>(env_var: &str) -> PentaractResult<T> {
        env::var(env_var)
            .map_err(|_| PentaractError::EnvConfigLoadingError(env_var.to_owned()))?
            .parse::<T>()
            .map_err(|_| PentaractError::EnvVarParsingError(env_var.to_owned()))
    }
}
