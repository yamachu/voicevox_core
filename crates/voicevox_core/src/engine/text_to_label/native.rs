use crate::engine::text_to_label::shared::*;

use ::open_jtalk::*;
use std::path::Path;

pub struct OpenJtalk {
    mecab: ManagedResource<Mecab>,
    njd: ManagedResource<Njd>,
    jpcommon: ManagedResource<JpCommon>,
    dict_loaded: bool,
}

impl TextToLabel for OpenJtalk {
    fn initialize() -> Self {
        Self {
            mecab: ManagedResource::initialize(),
            njd: ManagedResource::initialize(),
            jpcommon: ManagedResource::initialize(),
            dict_loaded: false,
        }
    }

    fn extract_fullcontext(&mut self, text: impl AsRef<str>) -> Result<Vec<String>> {
        let result = self.extract_fullcontext_non_reflesh(text);
        self.jpcommon.refresh();
        self.njd.refresh();
        self.mecab.refresh();
        result
    }

    fn load(&mut self, mecab_dict_dir: impl AsRef<Path>) -> Result<()> {
        let result = self.mecab.load(mecab_dict_dir.as_ref());
        if result {
            self.dict_loaded = true;
            Ok(())
        } else {
            self.dict_loaded = false;
            Err(OpenJtalkError::Load {
                mecab_dict_dir: mecab_dict_dir.as_ref().into(),
            })
        }
    }

    fn dict_loaded(&self) -> bool {
        self.dict_loaded
    }
}

impl OpenJtalk {
    fn extract_fullcontext_non_reflesh(&mut self, text: impl AsRef<str>) -> Result<Vec<String>> {
        let mecab_text =
            text2mecab(text.as_ref()).map_err(|e| OpenJtalkError::ExtractFullContext {
                text: text.as_ref().into(),
                source: Some(e.into()),
            })?;
        if self.mecab.analysis(mecab_text) {
            self.njd.mecab2njd(
                self.mecab
                    .get_feature()
                    .ok_or(OpenJtalkError::ExtractFullContext {
                        text: text.as_ref().into(),
                        source: None,
                    })?,
                self.mecab.get_size(),
            );
            self.njd.set_pronunciation();
            self.njd.set_digit();
            self.njd.set_accent_phrase();
            self.njd.set_accent_type();
            self.njd.set_unvoiced_vowel();
            self.njd.set_long_vowel();
            self.jpcommon.njd2jpcommon(&self.njd);
            self.jpcommon.make_label();
            self.jpcommon
                .get_label_feature_to_iter()
                .ok_or_else(|| OpenJtalkError::ExtractFullContext {
                    text: text.as_ref().into(),
                    source: None,
                })
                .map(|iter| iter.map(|s| s.to_string()).collect())
        } else {
            Err(OpenJtalkError::ExtractFullContext {
                text: text.as_ref().into(),
                source: None,
            })
        }
    }
}
