use konnektoren_yew::i18n::I18nConfig;
use std::collections::HashMap;

pub mod i18n_macro;

pub const LANGUAGE_KEY: &str = "selected_language";

pub const LANGUAGES: [&str; 9] = ["en", "ua", "ar", "de", "cn", "pl", "tr", "es", "vi"];

pub fn translations() -> HashMap<String, serde_json::Value> {
    let mut translations = HashMap::new();

    let en = serde_json::from_str(include_str!("../../assets/i18n/en.json")).unwrap();
    let de = serde_json::from_str(include_str!("../../assets/i18n/de.json")).unwrap();
    let ua = serde_json::from_str(include_str!("../../assets/i18n/ua.json")).unwrap();
    let cn = serde_json::from_str(include_str!("../../assets/i18n/cn.json")).unwrap();
    let ar = serde_json::from_str(include_str!("../../assets/i18n/ar.json")).unwrap();
    let pl = serde_json::from_str(include_str!("../../assets/i18n/pl.json")).unwrap();
    let tr = serde_json::from_str(include_str!("../../assets/i18n/tr.json")).unwrap();
    let es = serde_json::from_str(include_str!("../../assets/i18n/es.json")).unwrap();
    let vi = serde_json::from_str(include_str!("../../assets/i18n/vi.json")).unwrap();

    translations.insert("en".to_string(), en);
    translations.insert("de".to_string(), de);
    translations.insert("ua".to_string(), ua);
    translations.insert("cn".to_string(), cn);
    translations.insert("ar".to_string(), ar);
    translations.insert("pl".to_string(), pl);
    translations.insert("tr".to_string(), tr);
    translations.insert("es".to_string(), es);
    translations.insert("vi".to_string(), vi);
    translations
}

pub fn translation_config() -> I18nConfig {
    let mut config = I18nConfig::default();
    config.supported_languages = LANGUAGES.to_vec();
    config.default_language = "en".to_string();

    config.merge_translation(
        "en",
        serde_json::from_str(include_str!("../../assets/i18n/en.json")).unwrap(),
    );
    config.merge_translation(
        "de",
        serde_json::from_str(include_str!("../../assets/i18n/de.json")).unwrap(),
    );
    config.merge_translation(
        "ua",
        serde_json::from_str(include_str!("../../assets/i18n/ua.json")).unwrap(),
    );
    config.merge_translation(
        "cn",
        serde_json::from_str(include_str!("../../assets/i18n/cn.json")).unwrap(),
    );
    config.merge_translation(
        "ar",
        serde_json::from_str(include_str!("../../assets/i18n/ar.json")).unwrap(),
    );
    config.merge_translation(
        "pl",
        serde_json::from_str(include_str!("../../assets/i18n/pl.json")).unwrap(),
    );
    config.merge_translation(
        "tr",
        serde_json::from_str(include_str!("../../assets/i18n/tr.json")).unwrap(),
    );
    config.merge_translation(
        "es",
        serde_json::from_str(include_str!("../../assets/i18n/es.json")).unwrap(),
    );
    config.merge_translation(
        "vi",
        serde_json::from_str(include_str!("../../assets/i18n/vi.json")).unwrap(),
    );
    config
}

pub fn supported_language(lang: Option<&str>) -> Option<String> {
    match lang {
        Some(lang) => {
            if LANGUAGES.contains(&lang) {
                Some(lang.to_string())
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn flag(lang: &'static str) -> &'static str {
    match lang {
        "en" => "🇺🇸",
        "de" => "🇩🇪",
        "ua" => "🇺🇦",
        "cn" => "🇨🇳",
        "ar" => "🇸🇦",
        "pl" => "🇵🇱",
        "tr" => "🇹🇷",
        "es" => "🇪🇸",
        "vi" => "🇻🇳",
        _ => "🌐",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_language() {
        assert_eq!(supported_language(Some("en")), Some("en".to_string()));
        assert_eq!(supported_language(Some("ua")), Some("ua".to_string()));
        assert_eq!(supported_language(Some("de")), Some("de".to_string()));
        assert_eq!(supported_language(Some("cn")), Some("cn".to_string()));
        assert_eq!(supported_language(Some("ar")), Some("ar".to_string()));
        assert_eq!(supported_language(Some("pl")), Some("pl".to_string()));
        assert_eq!(supported_language(Some("tr")), Some("tr".to_string()));
        assert_eq!(supported_language(Some("es")), Some("es".to_string()));
        assert_eq!(supported_language(Some("vi")), Some("vi".to_string()));
        assert_eq!(supported_language(Some("fr")), None);
        assert_eq!(supported_language(None), None);
    }

    #[test]
    fn test_flag() {
        assert_eq!(flag("en"), "🇺🇸");
        assert_eq!(flag("de"), "🇩🇪");
        assert_eq!(flag("ua"), "🇺🇦");
        assert_eq!(flag("cn"), "🇨🇳");
        assert_eq!(flag("ar"), "🇸🇦");
        assert_eq!(flag("pl"), "🇵🇱");
        assert_eq!(flag("tr"), "🇹🇷");
        assert_eq!(flag("es"), "🇪🇸");
        assert_eq!(flag("fr"), "🌐");
        assert_eq!(flag("vi"), "🇻🇳");
    }
}
