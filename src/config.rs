use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Category {
    Documents,
    Pictures,
}

impl Category {
    pub fn name(&self) -> &'static str {
        match self {
            Category::Documents => "Documents",
            Category::Pictures => "Pictures",
        }
    }
}

pub struct Config {
    pub source_dir: PathBuf,
    pub docs_dir: PathBuf,
    pub pics_dir: PathBuf,
    pub dry_run: bool,
    pub watch: bool,
    pub interval_secs: u64,
}

impl Config {
    pub fn default_from_env() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let home_path = PathBuf::from(home);

        Config {
            source_dir: home_path.join("Downloads"),
            docs_dir: home_path.join("Documents"),
            pics_dir: home_path.join("Pictures"),
            dry_run: false,
            watch: false,
            interval_secs: 3,
        }
    }

    pub fn destination_for(&self, category: Category) -> &Path {
        match category {
            Category::Documents => &self.docs_dir,
            Category::Pictures => &self.pics_dir,
        }
    }
}

pub fn is_temporary_download(ext: &str) -> bool {
    matches!(
        ext.to_lowercase().as_str(),
        "crdownload" | "part" | "tmp" | "temp" | "download" | "aria2"
    )
}

pub fn classify_extension(ext: &str) -> Option<Category> {
    match ext.to_lowercase().as_str() {
        "pdf" | "docx" | "doc" | "txt" | "odt" | "xlsx" | "xls" | "pptx" | "ppt" | "csv"
        | "rtf" | "epub" => Some(Category::Documents),

        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" | "tiff" | "ico" => {
            Some(Category::Pictures)
        }

        _ => None,
    }
}
