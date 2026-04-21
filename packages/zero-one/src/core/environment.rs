use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Environment {}

impl Environment {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();
        let env = envy::from_env::<Self>()?;
        Ok(env)
    }
}
