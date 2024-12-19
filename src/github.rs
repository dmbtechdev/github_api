use octocrab::Octocrab;
use tracing::info;
use anyhow::Result;


#[derive(Debug)]
pub struct GitHubClientBuilder {
    pub octocrab: Octocrab,
}

impl GitHubClientBuilder {
    
    pub async fn new(token: String) -> Result<Self> {

        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()?;

            info!("Github Api Client initialized");

        Ok(Self {
            octocrab,
        })
    }

    pub fn client(self) -> Octocrab {
        self.octocrab
    }
}