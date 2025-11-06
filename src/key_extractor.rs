//! Key extraction trait for sorting with custom key functions.

/// Trait for extracting sort keys from elements.
///
/// This trait allows tilesort to work with both elements that are directly
/// comparable (identity extraction) and elements that need a custom key function.
pub trait KeyExtractor<T, K> {
    /// Extract the sort key from an element.
    fn extract_key(&self, item: &T) -> K;
}

/// Identity key extractor - the element is its own key.
///
/// This is used when no custom key function is provided.
pub struct IdentityKey;

impl<T: Clone> KeyExtractor<T, T> for IdentityKey {
    fn extract_key(&self, item: &T) -> T {
        item.clone()
    }
}

/// Blanket implementation for function-based key extraction.
///
/// This allows any closure or function pointer that takes `&T` and returns `K`
/// to be used as a key extractor.
impl<T, K, F> KeyExtractor<T, K> for F
where
    F: Fn(&T) -> K,
{
    fn extract_key(&self, item: &T) -> K {
        self(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_key() {
        let extractor = IdentityKey;
        assert_eq!(extractor.extract_key(&42), 42);
        assert_eq!(extractor.extract_key(&"hello"), "hello");
    }

    #[test]
    fn test_function_key() {
        let extractor = |x: &i32| x.abs();
        assert_eq!(extractor.extract_key(&-42), 42);
        assert_eq!(extractor.extract_key(&10), 10);
    }

    #[test]
    fn test_complex_key() {
        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };

        let by_age = |p: &Person| p.age;
        assert_eq!(by_age.extract_key(&person), 30);

        let by_name_len = |p: &Person| p.name.len();
        assert_eq!(by_name_len.extract_key(&person), 5);
    }
}
