use std::collections::HashMap;

use super::{REMOTE, SERVERS};

pub(super) fn mockup_chat() {
    SERVERS.get_or_init(|| {
        HashMap::from([
            (
                "127.1.0.22".to_owned(),
                "la1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.24".to_owned(),
                "la2.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.17".to_owned(),
                "sa1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.16".to_owned(),
                "ru1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.9".to_owned(),
                "jp1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.13".to_owned(),
                "na2.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.8".to_owned(),
                "euw1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.12".to_owned(),
                "la2.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.5".to_owned(),
                "ru1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.6".to_owned(),
                "eu3.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.20".to_owned(),
                "sa4.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.14".to_owned(),
                "oc1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.23".to_owned(),
                "br.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.19".to_owned(),
                "sa3.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.2".to_owned(),
                "as2.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.25".to_owned(),
                "us2.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.15".to_owned(),
                "pbe1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.4".to_owned(),
                "br.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.10".to_owned(),
                "kr1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.7".to_owned(),
                "eun1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.18".to_owned(),
                "sa2.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.11".to_owned(),
                "la1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.3".to_owned(),
                "jp1.chat.si.riotgames.com".to_owned(),
            ),
            (
                "127.1.0.21".to_owned(),
                "tr1.chat.si.riotgames.com".to_owned(),
            ),
        ])
    });
    REMOTE.get_or_init(|| 5223);
}