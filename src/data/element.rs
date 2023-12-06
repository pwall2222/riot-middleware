use minidom::Element;
use thiserror::Error;

pub static CLIENT: &'static str = "jabber:client";
pub static ROSTER: &'static str = "jabber:iq:riotgames:roster";
pub static SESSION: &'static str = "urn:ietf:params:xml:ns:xmpp-session";

#[derive(Debug, Error)]
pub enum ElementError {
    #[error("The element child doesn't exist")]
    MissingElement(String),
    #[error("The element attribute doesn't exist")]
    MissingAttribute((String, String)),
    #[error("The text for the element is empty")]
    MissingText(String),
}

pub fn get_text(el: Result<&Element, ElementError>) -> Result<String, ElementError> {
    let el = el?;
    let text = el.text();
    if text.is_empty() {
        return Err(ElementError::MissingText(el.name().to_string()));
    }
    Ok(text)
}

pub fn get_attr(el: Result<&Element, ElementError>, attr: &str) -> Result<String, ElementError> {
    let el = el?;
    Ok(el
        .attr(attr)
        .ok_or(ElementError::MissingAttribute((
            el.name().to_string(),
            attr.to_string(),
        )))?
        .to_string())
}

pub fn get_child<'a>(
    el: &'a Element,
    name: &str,
    namespace: &str,
) -> Result<&'a Element, ElementError> {
    el.get_child(name, namespace)
        .ok_or(ElementError::MissingElement(name.to_string()))
}

pub fn parse(data: String) -> Option<Element> {
    let data = format!("<stream xmlns='{}'>{}</stream>", CLIENT, data);
    let xml: Element = data.parse().ok()?;
    if xml.children().count() > 1 {
        return Some(xml);
    }
    Some(xml.children().next()?.clone())
}
