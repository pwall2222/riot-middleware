use super::{get_attr, get_child, get_text, ElementError, CLIENT};
use base64::{engine::general_purpose, Engine as _};
use minidom::Element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Presence {
    pub from: String,
    pub to: String,
    pub puuid: String,
    pub show: Option<String>,
    pub platform: Option<String>,
    pub last_online: Option<String>,
    pub valorant_presence: Option<ValorantPresence>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ValorantPresence {
    pub is_valid: bool,
    pub session_loop_state: String,
    pub party_owner_session_loop_state: String,
    pub custom_game_name: String,
    pub custom_game_team: String,
    pub party_owner_match_map: String,
    pub party_owner_match_current_team: String,
    pub party_owner_match_score_ally_team: i64,
    pub party_owner_match_score_enemy_team: i64,
    pub party_owner_provisioning_flow: String,
    pub provisioning_flow: String,
    pub match_map: String,
    pub party_id: String,
    pub is_party_owner: bool,
    pub party_state: String,
    pub party_accessibility: String,
    pub max_party_size: i64,
    pub queue_id: String,
    #[serde(rename = "partyLFM")]
    pub party_lfm: bool,
    pub party_client_version: String,
    pub party_size: i64,
    pub tournament_id: String,
    pub roster_id: String,
    pub party_version: i64,
    pub queue_entry_time: String,
    pub player_card_id: String,
    pub player_title_id: String,
    pub preferred_level_border_id: String,
    pub account_level: i64,
    pub competitive_tier: i64,
    pub leaderboard_position: i64,
    pub is_idle: bool,
}

fn get_valorant_presence(i: &Element) -> Result<ValorantPresence, ElementError> {
    let games = get_child(i, "games", CLIENT)?;
    let valorant = get_child(games, "valorant", CLIENT)?;
    let presence = get_child(valorant, "p", CLIENT);
    let b64 = get_text(presence)?;
    let b64 = b64.trim();
    let raw_json = general_purpose::STANDARD.decode(b64).unwrap();
    let welcome: ValorantPresence = serde_json::from_slice(&raw_json).unwrap();
    Ok(welcome)
}

/// Extract UUID with the `from` param of xmpp
///
/// Possible cases:
///
/// `{match_uuid}-{blue|all}@{server}/{puuid}`
///
/// `{puuid}@{server}`
///
/// `{puuid}@{server}/RC-{num}`
fn get_puuid(s: &str) -> &str {
    let (username, uri) = s.split_once("@").unwrap();
    // Check whether the "username" half is a match uuid
    if username.len() > 36 {
        // Return the player in the uri path
        return uri.split_once("/").unwrap().1;
    }
    username
}

impl TryFrom<&Element> for Presence {
    fn try_from(i: &Element) -> Result<Self, Self::Error> {
        let from = get_attr(Ok(i), "from")?;
        let to = get_attr(Ok(i), "to")?;
        let puuid = get_puuid(&from).to_string();
        let platform = get_text(get_child(i, "platform", CLIENT)).ok();
        let show = get_text(get_child(i, "show", CLIENT)).ok();
        let last_online = get_text(get_child(i, "last_online", CLIENT)).ok();
        let valorant_presence = get_valorant_presence(i).ok();

        Ok(Presence {
            from,
            to,
            puuid,
            show,
            platform,
            last_online,
            valorant_presence,
        })
    }

    type Error = ElementError;
}
