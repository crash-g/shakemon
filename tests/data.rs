#[derive(serde::Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub description: String,
}

pub mod pokeapi {
    pub const ENDPOINT: &str = "/api/v2/pokemon-species";

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Pokemon {
        pub flavor_text_entries: Vec<FlavorTextEntry>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct FlavorTextEntry {
        pub flavor_text: String,
        pub language: Language,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Language {
        pub name: String,
    }
}

pub mod shakespeare {
    pub const ENDPOINT: &str = "/translate/shakespeare";

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Translation {
        pub contents: TranslationContents,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct TranslationContents {
        pub translated: String,
    }
}
