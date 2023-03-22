use figment::{
  providers::{Toml, Data, Format},
  error::{Result},
  value::{Dict, Map},
  Profile, Provider, Figment,
};

pub struct ScarbProvider {
  provider: Data<Toml>,
}

impl ScarbProvider {
  pub fn new(path: &str) -> Self {
      Self {
        provider: Toml::file(path)
      }
  }
}

impl Provider for ScarbProvider {
  fn metadata(&self) -> figment::Metadata {
      self.provider.metadata()
  }

  fn data(&self) -> Result<Map<Profile, Dict>> {
    let pair = self.provider.data()?.pop_first();
    let mut map = Map::new();

    if let Some((profile, _)) = pair {
        map.insert(profile, Figment::from(&self.provider).extract_inner("tool.nile")?);
    }

    Ok(map)
  }
}
