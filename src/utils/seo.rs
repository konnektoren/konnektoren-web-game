use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlMetaElement;

pub fn update_meta_tags(
    title: &str,
    description: &str,
    keywords: &str,
    site_url: &str,
    image_url: &str,
    language: &str,
) {
    let head = document().head().unwrap();

    // Clear existing meta tags
    let meta_tags = head.query_selector_all("meta").unwrap();
    // Convert NodeList to an iterator
    for i in 0..meta_tags.length() {
        if let Ok(meta_tag) = meta_tags.item(i).unwrap().dyn_into::<HtmlMetaElement>() {
            head.remove_child(&meta_tag).unwrap();
        }
    }

    // Title
    document().set_title(&format!("{} - Konnektoren", title));

    // Meta Description
    let meta_description = document()
        .create_element("meta")
        .unwrap()
        .dyn_into::<HtmlMetaElement>()
        .unwrap();
    meta_description
        .set_attribute("name", "description")
        .unwrap();
    meta_description
        .set_attribute("content", description)
        .unwrap();
    head.append_child(&meta_description).unwrap();

    // Meta Keywords
    let meta_keywords = document()
        .create_element("meta")
        .unwrap()
        .dyn_into::<HtmlMetaElement>()
        .unwrap();
    meta_keywords.set_attribute("name", "keywords").unwrap();
    meta_keywords.set_attribute("content", keywords).unwrap();
    head.append_child(&meta_keywords).unwrap();

    // Language
    let meta_language = document()
        .create_element("meta")
        .unwrap()
        .dyn_into::<HtmlMetaElement>()
        .unwrap();
    meta_language.set_attribute("name", "language").unwrap();
    meta_language.set_attribute("content", language).unwrap();
    head.append_child(&meta_language).unwrap();

    // Open Graph
    let og_title = document()
        .create_element("meta")
        .unwrap()
        .dyn_into::<HtmlMetaElement>()
        .unwrap();
    og_title.set_attribute("property", "og:title").unwrap();
    og_title.set_attribute("content", title).unwrap();
    head.append_child(&og_title).unwrap();

    let og_description = document()
        .create_element("meta")
        .unwrap()
        .dyn_into::<HtmlMetaElement>()
        .unwrap();
    og_description
        .set_attribute("property", "og:description")
        .unwrap();
    og_description
        .set_attribute("content", description)
        .unwrap();
    head.append_child(&og_description).unwrap();

    let og_url = document()
        .create_element("meta")
        .unwrap()
        .dyn_into::<HtmlMetaElement>()
        .unwrap();
    og_url.set_attribute("property", "og:url").unwrap();
    og_url.set_attribute("content", site_url).unwrap();
    head.append_child(&og_url).unwrap();

    let og_image = document()
        .create_element("meta")
        .unwrap()
        .dyn_into::<HtmlMetaElement>()
        .unwrap();
    og_image.set_attribute("property", "og:image").unwrap();
    og_image.set_attribute("content", image_url).unwrap();
    head.append_child(&og_image).unwrap();

    let og_locale = document()
        .create_element("meta")
        .unwrap()
        .dyn_into::<HtmlMetaElement>()
        .unwrap();
    og_locale.set_attribute("property", "og:locale").unwrap();
    og_locale
        .set_attribute(
            "content",
            &format!("{}_{}", language, language.to_uppercase()),
        )
        .unwrap();
    head.append_child(&og_locale).unwrap();
}
