use anyhow::Result;
use rman::RiotManifest;

pub fn testa() -> Result<()> {
    let path = r"C:\ProgramData\Riot Games\Metadata\Riot Client\Riot Client.manifest";

    let manifest = RiotManifest::from_path(path, None)?;

    dbg!(manifest.data.files);
    Ok(())
}
