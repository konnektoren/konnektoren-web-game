use konnektoren_core::marketplace::{Product, ProductCatalog};
use std::collections::HashMap;
use strsim::normalized_levenshtein;

pub struct SearchProductCatalog {
    product_index: HashMap<String, Vec<Product>>,
}

impl SearchProductCatalog {
    pub fn new(catalog: &ProductCatalog) -> Self {
        let mut product_index = HashMap::new();

        for product in &catalog.products {
            let name_words = Self::tokenize(&product.name);
            let desc_words = Self::tokenize(&product.description);
            let tag_words = product
                .tags
                .iter()
                .flat_map(|tag| Self::tokenize(tag))
                .collect::<Vec<_>>();

            let id_word = product.id.clone().unwrap_or_default();

            let all_words: Vec<String> = name_words
                .into_iter()
                .chain(desc_words)
                .chain(tag_words)
                .chain(std::iter::once(id_word))
                .collect();

            for word in all_words {
                product_index
                    .entry(word)
                    .or_insert_with(Vec::new)
                    .push(product.clone());
            }
        }

        Self { product_index }
    }

    pub fn search(&self, search_term: &str) -> Vec<Product> {
        let search_words = Self::tokenize(search_term);
        let mut results = HashMap::new();

        for word in &search_words {
            for (index_word, products) in &self.product_index {
                let similarity = Self::calculate_similarity(word, index_word);
                if similarity > 0.7 {
                    // Similarity threshold
                    for product in products {
                        results
                            .entry(product.id.clone())
                            .or_insert_with(|| (product.clone(), 0.0))
                            .1 += similarity;
                    }
                }
            }
        }

        let mut sorted_results: Vec<_> = results.into_values().collect();
        sorted_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        sorted_results
            .into_iter()
            .map(|(product, _)| product)
            .collect()
    }

    pub fn filtered(&self, search_term: &str) -> ProductCatalog {
        let products = self.search(search_term);
        let mut catalog = ProductCatalog::new("Filtered Catalog".to_string());
        for product in products {
            catalog.add_product(product);
        }
        catalog
    }

    pub fn get_suggestion(&self, partial_word: &str) -> Option<String> {
        if partial_word.is_empty() {
            return None;
        }

        let partial_word = partial_word.to_lowercase();
        self.product_index
            .keys()
            .filter(|word| {
                word.to_lowercase().contains(&partial_word)
                    || normalized_levenshtein(&partial_word, &word.to_lowercase()) > 0.7
            })
            .max_by(|a, b| {
                let a_sim = normalized_levenshtein(&partial_word, &a.to_lowercase());
                let b_sim = normalized_levenshtein(&partial_word, &b.to_lowercase());
                a_sim.partial_cmp(&b_sim).unwrap()
            })
            .cloned()
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|word| word.to_lowercase())
            .filter(|word| !word.is_empty()) // Filter empty strings
            .collect()
    }

    fn calculate_similarity(word1: &str, word2: &str) -> f64 {
        if word1 == word2 {
            return 1.0;
        }
        normalized_levenshtein(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use konnektoren_core::marketplace::{Product, ProductCatalog};

    #[test]
    fn test_search_products() {
        let mut catalog = ProductCatalog::new("TestCatalog".to_string());
        let product1 = Product::new("Konnektoren".to_string(), "A test product".to_string());
        let product2 = Product::new(
            "Verbs Mastery".to_string(),
            "Learn verb conjugation".to_string(),
        );
        let product3 = Product::new(
            "Grammar Rules".to_string(),
            "Detailed grammar rules".to_string(),
        );

        catalog.add_product(product1.clone());
        catalog.add_product(product2.clone());
        catalog.add_product(product3.clone());

        let search_product_catalog = SearchProductCatalog::new(&catalog);

        // Search by name
        let products = search_product_catalog.search("Konnektoren");
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].name, product1.name);

        // Search by description
        let products = search_product_catalog.search("verb");
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].name, product2.name);

        // Test suggestion
        let suggestion = search_product_catalog.get_suggestion("Konnekt");
        assert_eq!(suggestion.unwrap(), "konnektoren");
    }
}
