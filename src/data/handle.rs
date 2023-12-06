use minidom::Element;

use super::{parse, ElementError, Friend, Presence, Session, ROSTER};
use crate::server::{FRIENDS, PRESENCES, SESSION};

fn handle_presence(el: &Element) -> Result<(), ElementError> {
    let presence = Presence::try_from(el)?;
    let puuid = presence.puuid.to_string();
    PRESENCES.write().unwrap().insert(puuid, presence);
    Ok(())
}

fn handle_session(el: &Element) -> Result<(), ElementError> {
    let session = Session::try_from(el)?;
    *SESSION.write().unwrap() = session;
    Ok(())
}

fn handle_friend(el: &Element) -> Result<(), ElementError> {
    let friend = Friend::try_from(el)?;
    FRIENDS.write().unwrap().push(friend);
    Ok(())
}

fn handle_query(el: &Element) -> Result<(), ElementError> {
    if !el.is("query", ROSTER) {
        return Ok(());
    }
    for child in el.children() {
        let _ = handle_friend(child);
    }
    Ok(())
}

fn handle_iq(el: &Element) -> Result<(), ElementError> {
    let el = el
        .children()
        .next()
        .ok_or(ElementError::MissingElement("child".to_string()))?;
    match el.name() {
        "session" => handle_session(el),
        "query" => handle_query(el),
        _ => Ok(()),
    }
}

fn handle_element(el: &Element) {
    let _ = match el.name() {
        "presence" => handle_presence(el),
        "iq" => handle_iq(el),
        _ => Ok(()),
    };
}

pub fn handle_data(data: &Vec<u8>) -> Option<()> {
    let root = parse(String::from_utf8(data.to_vec()).unwrap())?;
    match root.name() {
        "stream" => {
            for c in root.children() {
                handle_element(c);
            }
        }
        _ => handle_element(&root),
    };
    None
}
