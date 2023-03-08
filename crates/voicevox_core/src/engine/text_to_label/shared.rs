use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum OpenJtalkError {
    #[error("open_jtalk load error")]
    Load { mecab_dict_dir: PathBuf },
    #[error("open_jtalk extract_fullcontext error")]
    ExtractFullContext {
        text: String,
        #[source]
        source: Option<anyhow::Error>,
    },
}

impl PartialEq for OpenJtalkError {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (
                Self::Load {
                    mecab_dict_dir: mecab_dict_dir1,
                },
                Self::Load {
                    mecab_dict_dir: mecab_dict_dir2,
                },
            ) => mecab_dict_dir1 == mecab_dict_dir2,
            (
                Self::ExtractFullContext {
                    text: text1,
                    source: source1,
                },
                Self::ExtractFullContext {
                    text: text2,
                    source: source2,
                },
            ) => (text1, by_display(source1)) == (text2, by_display(source2)),
            _ => false,
        };

        fn by_display(source: &Option<anyhow::Error>) -> impl PartialEq {
            source.as_ref().map(|e| e.to_string())
        }
    }
}

pub type Result<T> = std::result::Result<T, OpenJtalkError>;

pub trait TextToLabel {
    fn initialize() -> Self;
    fn extract_fullcontext(&mut self, text: impl AsRef<str>) -> Result<Vec<String>>;
    fn load(&mut self, mecab_dict_dir: impl AsRef<Path>) -> Result<()>;
    fn dict_loaded(&self) -> bool;
}
