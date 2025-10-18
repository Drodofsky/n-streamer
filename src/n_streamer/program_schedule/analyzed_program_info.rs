use crate::n_streamer::program_schedule::parsed_program_info::Item;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnalyzedProgramInfo {
    pub program_master_id: i64,
    pub program_name: String,
    pub genre: Option<String>,
    pub logo_link: Option<String>,
    pub link: Option<String>,
    pub synopsis: Option<String>,
}

impl From<Item> for AnalyzedProgramInfo {
    fn from(value: Item) -> Self {
        AnalyzedProgramInfo {
            program_master_id: value.program_master_id,
            program_name: value.program_name,
            genre: value.genre,
            logo_link: value.logo_link,
            link: value.link,
            synopsis: value.synopsis,
        }
    }
}
