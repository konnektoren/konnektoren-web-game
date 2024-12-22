use crate::utils::translation::LANGUAGE_KEY;
use gloo::storage::{LocalStorage, Storage};
use yew_i18n::{YewI18n, YewI18nConfig};

use super::{translations, LANGUAGES};

pub fn selected_language() -> String {
    LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string())
}

pub fn get_translation(text: &str, lang: Option<&str>) -> String {
    let language = lang
        .map(|l| l.to_string())
        .unwrap_or_else(selected_language);

    let config = YewI18nConfig {
        supported_languages: LANGUAGES.to_vec(),
        translations: translations(),
    };

    let mut i18n = YewI18n::new(config, translations()).unwrap();
    i18n.set_translation_language(&language).ok();
    i18n.t(text)
}

#[deprecated(since = "0.1.0", note = "Please use the `i18n.t` instead")]
#[macro_export]
macro_rules! i18n {
    ($text:expr) => {{
        $crate::utils::translation::i18n_macro::get_translation($text, None)
    }};
    ($text:expr, $lang:expr) => {{
        $crate::utils::translation::i18n_macro::get_translation($text, Some($lang))
    }};
}
