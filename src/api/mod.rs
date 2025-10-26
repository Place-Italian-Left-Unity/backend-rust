use wplace_core_library::template_data::TemplateData;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtListData {
    name: String,
    location: String,
    file_name: String,
}

impl ArtListData {
    pub fn from_template_data(v: &TemplateData) -> Self {
        Self {
            name: v.get_name().to_string(),
            location: v.get_nominatim_data().get_display_name().to_string(),
            file_name: v.get_file_name().to_string(),
        }
    }
}
