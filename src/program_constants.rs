use serde::Deserialize;
use wplace_core_library::{
    image_data::ImageData, template_data::TemplateData, tile_coords::TileCoords,
};

pub struct ProgramConstants {
    pub templates_path: String,
    pub server_threads: u8,
    pub templates_data: Vec<TemplateData>,
}

impl Default for ProgramConstants {
    fn default() -> Self {
        Self {
            templates_path: String::from("./templates/"),
            server_threads: 4,
            templates_data: Vec::with_capacity(32),
        }
    }
}

impl ProgramConstants {
    pub fn lazy_evaluate() -> Self {
        let mut out = ProgramConstants::default();

        println!("+++ Reading CLI Args +++");

        for arg in std::env::args() {
            let mut split = arg.split('=');
            match (split.next(), split.next()) {
                (Some("template_path"), Some(v))
                | (Some("templates"), Some(v))
                | (Some("tpl"), Some(v)) => out.templates_path = v.to_string(),
                (Some("threads"), Some(v)) | (Some("td"), Some(v)) => match v.parse() {
                    Ok(v) => out.server_threads = v,
                    Err(_) => continue,
                },
                _ => continue,
            }
        }

        println!("CLI Templates Path: {}", out.templates_path);
        println!("CLI Server Threads: {}", out.server_threads);

        if !std::path::Path::new(&out.templates_path).exists() {
            panic!("Templates path doesn't exist");
        }

        let template_metadata_path = format!("{}template_metadata.json", out.templates_path);
        if !std::path::Path::new(&template_metadata_path).exists() {
            panic!("Templates metadata file doesn't exist");
        }

        let wplace_template_metadata: Vec<WplaceTemplateMetadata> = serde_json::from_reader(
            std::fs::File::open(&template_metadata_path)
                .expect("Couldn't open Template metadata file"),
        )
        .expect("Couldn't read template metadata file properly");

        out.templates_data = wplace_template_metadata
            .into_iter()
            .map(|metadata| {
                let file_path = format!("{}{}", out.templates_path, metadata.file_name);
                TemplateData::new(
                    metadata.name,
                    TileCoords::parse_tile_coords_string(&metadata.coords).unwrap(),
                    match ImageData::new(std::path::Path::new(&file_path)) {
                        Ok(v) => v,
                        Err(e) => panic!("Couldn't read {file_path}: {e}")
                    }
                )
            })
            .collect();

        out
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WplaceTemplateMetadata {
    name: String,
    file_name: String,
    coords: String,
    alliance: String,
}
