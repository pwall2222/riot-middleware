use super::{get_attr, get_child, get_text, ElementError, ROSTER};
use minidom::Element;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Friend {
    pub status: String,
    pub puuid: String,
    pub riot_name: String,
    pub riot_tag: String,
    pub riot_user: String,
    pub name: Option<String>,
    pub state: Option<String>,
    pub note: Option<String>,
    pub last_online: Option<String>,
    pub lol_name: Option<String>,
}

impl TryFrom<&Element> for Friend {
    fn try_from(i: &Element) -> Result<Self, Self::Error> {
        let puuid = get_attr(Ok(i), "puuid")?;
        let status = get_attr(Ok(i), "subscription")?;
        let name = get_attr(Ok(i), "name").ok();
        let id = get_child(i, "id", ROSTER)?;
        let riot_name = get_attr(Ok(id), "name")?;
        let riot_tag = get_attr(Ok(id), "tagline")?;
        let riot_user = format!("{}#{}", riot_name, riot_tag);
        let state = get_text(get_child(i, "state", ROSTER)).ok();
        let note = get_text(get_child(i, "note", ROSTER)).ok();
        let last_online = get_text(get_child(i, "last_online", ROSTER)).ok();
        let lol_name = get_attr(get_child(i, "lol", ROSTER), "name").ok();
        Ok(Friend {
            status,
            puuid,
            riot_name,
            riot_tag,
            riot_user,
            name,
            state,
            note,
            last_online,
            lol_name,
        })
    }

    type Error = ElementError;
}
