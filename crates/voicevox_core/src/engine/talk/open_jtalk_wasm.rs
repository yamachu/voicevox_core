/// Link-test substitute only; it deliberately does not analyze text.
pub(super) trait FullcontextExtractor {
    fn extract_fullcontext(&self, text: &str) -> anyhow::Result<Vec<String>>;
}

pub(crate) mod blocking {
    use camino::Utf8Path;

    use crate::assert::assert_send_sync;

    use super::{
        super::{AccentPhrase, extract_full_context_label},
        FullcontextExtractor,
    };

    /// Link-test substitute; text analysis is intentionally unavailable.
    #[derive(Clone, Debug)]
    pub struct OpenJtalk;

    impl OpenJtalk {
        /// Constructs the substitute without loading a dictionary.
        pub fn new(_: impl AsRef<Utf8Path>) -> crate::result::Result<Self> {
            Ok(Self)
        }

        /// Accepts the API call without modifying a dictionary.
        pub fn use_user_dict(&self, _: &crate::blocking::UserDict) -> crate::result::Result<()> {
            Ok(())
        }
    }

    impl FullcontextExtractor for OpenJtalk {
        fn extract_fullcontext(&self, _: &str) -> anyhow::Result<Vec<String>> {
            anyhow::bail!("text analysis is unavailable in the link-test substitute")
        }
    }

    impl crate::blocking::TextAnalyzer for OpenJtalk {
        fn analyze(&self, text: &str) -> anyhow::Result<Vec<AccentPhrase>> {
            if text.is_empty() {
                return Ok(Vec::new());
            }
            Ok(extract_full_context_label(self, text)?)
        }
    }

    assert_send_sync!(OpenJtalk);
}

pub(crate) mod nonblocking {
    use camino::Utf8Path;

    use crate::assert::assert_send_sync;

    /// Asynchronous wrapper for the link-test substitute.
    #[derive(Clone, Debug)]
    pub struct OpenJtalk(pub(in super::super) super::blocking::OpenJtalk);

    impl OpenJtalk {
        /// Constructs the substitute without loading a dictionary.
        pub async fn new(open_jtalk_dict_dir: impl AsRef<Utf8Path>) -> crate::result::Result<Self> {
            super::blocking::OpenJtalk::new(open_jtalk_dict_dir).map(Self)
        }

        /// Accepts the API call without modifying a dictionary.
        pub async fn use_user_dict(
            &self,
            _: &crate::nonblocking::UserDict,
        ) -> crate::result::Result<()> {
            Ok(())
        }
    }

    impl crate::nonblocking::TextAnalyzer for OpenJtalk {
        async fn analyze(&self, text: &str) -> anyhow::Result<Vec<crate::AccentPhrase>> {
            crate::blocking::TextAnalyzer::analyze(&self.0, text)
        }
    }

    assert_send_sync!(OpenJtalk);
}
