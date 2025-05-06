use std::fmt::Display;

use crate::settings::Settings;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub struct Info;

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let settings = Settings::get();
        let debug = settings.debug.unwrap_or_default();
        let server_port = settings.server_port;
        let server_url = format!("http://localhost:{server_port}");
        let branch = built_info::GIT_HEAD_REF.unwrap_or_default();
        let commit = built_info::GIT_COMMIT_HASH_SHORT.unwrap_or_default();
        writeln!(
            f,
            "{}",
            figlet_rs::FIGfont::standard()
                .unwrap()
                .convert(built_info::PKG_NAME)
                .unwrap()
        )?;
        writeln!(f, "Built info:")?;
        writeln!(f, "    Name:    {}", built_info::PKG_NAME)?;
        writeln!(f, "    Authors: {}", built_info::PKG_AUTHORS)?;
        writeln!(f, "    Version: {}", built_info::PKG_VERSION)?;
        writeln!(f, "    Branch:  {}", branch)?;
        writeln!(f, "    Commit:  {}", commit)?;
        writeln!(f, "    Build:   {}", built_info::RUSTC_VERSION)?;
        writeln!(f, "    OS:      {}", built_info::CFG_OS)?;
        writeln!(f, "    Family:  {}", built_info::CFG_FAMILY)?;
        writeln!(f, "    Arch:    {}", built_info::TARGET)?;
        writeln!(f, "    Endian:  {}", built_info::CFG_ENDIAN)?;
        writeln!(f, "    Profile: {}", built_info::PROFILE)?;
        if debug {
            writeln!(f, "    Configs: {}", settings.json_pretty(4))?;
        }
        writeln!(f, "    Server:  {}", server_url)?;
        writeln!(f, "    Api doc: {}", format!("{server_url}/docs"))?;
        writeln!(f, "    Api def: {}", format!("{server_url}/openapi.json"))?;
        Ok(())
    }
}
