use super::{get_attr, get_child, get_text, ElementError, SESSION};
use minidom::Element;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Session {
    pub riot_name: String,
    pub riot_tag: String,
    pub riot_user: String,
    pub timestamp: String,
    pub platform: Option<String>,
    pub lol_name: Option<String>,
    pub summoner_name: Option<String>,
}

impl TryFrom<&Element> for Session {
    fn try_from(i: &Element) -> Result<Self, Self::Error> {
        let id = get_child(i, "id", SESSION)?;
        let riot_name = get_attr(Ok(id), "name")?;
        let riot_tag = get_attr(Ok(id), "tagline")?;
        let riot_user = format!("{}#{}", riot_name, riot_tag);
        let ts = get_text(get_child(i, "ts", SESSION))?;
        let platform = get_text(get_child(i, "platform", SESSION)).ok();
        let lol_name = get_attr(get_child(i, "lol", SESSION), "name").ok();
        let summoner_name = get_text(get_child(i, "summoner_name", SESSION)).ok();
        Ok(Session {
            platform,
            riot_name,
            riot_tag,
            riot_user,
            timestamp: ts,
            lol_name,
            summoner_name,
        })
    }

    type Error = ElementError;
}

impl Session {
    pub fn new() -> Self {
        Session {
            riot_name: "".to_string(),
            riot_tag: "".to_string(),
            riot_user: "".to_string(),
            timestamp: "".to_string(),
            platform: None,
            lol_name: None,
            summoner_name: None,
        }
    }
}
