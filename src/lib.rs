//! Tilesort - A sorting algorithm optimized for datasets with pre-sorted contiguous blocks.
//!
//! This library provides efficient sorting for data consisting of non-overlapping,
//! pre-sorted contiguous blocks called "tiles".

mod key_extractor;
mod sorter;
mod tile_index;

pub use key_extractor::{IdentityKey, KeyExtractor};

// Rust sorting implementation (always available)

/// Sort a slice using the tilesort algorithm.
///
/// This function sorts elements in ascending order, optimized for data that consists
/// of pre-sorted contiguous blocks (tiles).
///
/// # Examples
///
/// ```
/// let mut data = vec![3, 4, 5, 1, 2, 6, 7, 8];
/// tilesort::tilesort(&mut data);
/// assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
pub fn tilesort<T: Ord + Clone>(data: &mut [T]) {
    sorter::tilesort_impl(data, false);
}

/// Sort a slice in descending order using the tilesort algorithm.
///
/// # Examples
///
/// ```
/// let mut data = vec![3, 4, 5, 1, 2, 6, 7, 8];
/// tilesort::tilesort_reverse(&mut data);
/// assert_eq!(data, vec![8, 7, 6, 5, 4, 3, 2, 1]);
/// ```
pub fn tilesort_reverse<T: Ord + Clone>(data: &mut [T]) {
    sorter::tilesort_impl(data, true);
}

/// Return a sorted copy of a slice using the tilesort algorithm.
///
/// This function does not modify the original slice and returns a new sorted vector.
///
/// # Examples
///
/// ```
/// let data = vec![3, 4, 5, 1, 2, 6, 7, 8];
/// let sorted = tilesort::tilesorted(&data);
/// assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// assert_eq!(data, vec![3, 4, 5, 1, 2, 6, 7, 8]); // Original unchanged
/// ```
pub fn tilesorted<T: Ord + Clone>(data: &[T]) -> Vec<T> {
    let mut result = data.to_vec();
    sorter::tilesort_impl(&mut result, false);
    result
}

/// Return a sorted copy of a slice in descending order using the tilesort algorithm.
///
/// This function does not modify the original slice and returns a new sorted vector.
///
/// # Examples
///
/// ```
/// let data = vec![3, 4, 5, 1, 2, 6, 7, 8];
/// let sorted = tilesort::tilesorted_reverse(&data);
/// assert_eq!(sorted, vec![8, 7, 6, 5, 4, 3, 2, 1]);
/// assert_eq!(data, vec![3, 4, 5, 1, 2, 6, 7, 8]); // Original unchanged
/// ```
pub fn tilesorted_reverse<T: Ord + Clone>(data: &[T]) -> Vec<T> {
    let mut result = data.to_vec();
    sorter::tilesort_impl(&mut result, true);
    result
}

/// Sort a slice using a custom key extraction function.
///
/// # Examples
///
/// ```
/// let mut data = vec![-5i32, -3, -1, 2, 4];
/// tilesort::tilesort_by_key(&mut data, |&x| x.abs());
/// assert_eq!(data, vec![-1, 2, -3, 4, -5]);
/// ```
pub fn tilesort_by_key<T, K, F>(data: &mut [T], key_fn: F)
where
    T: Clone,
    K: Ord + Clone,
    F: Fn(&T) -> K,
{
    sorter::tilesort_impl_with_key(data, key_fn, false);
}

/// Sort a slice in descending order using a custom key extraction function.
///
/// # Examples
///
/// ```
/// let mut data = vec![-5i32, -3, -1, 2, 4];
/// tilesort::tilesort_by_key_reverse(&mut data, |&x| x.abs());
/// assert_eq!(data, vec![-5, 4, -3, 2, -1]);
/// ```
pub fn tilesort_by_key_reverse<T, K, F>(data: &mut [T], key_fn: F)
where
    T: Clone,
    K: Ord + Clone,
    F: Fn(&T) -> K,
{
    sorter::tilesort_impl_with_key(data, key_fn, true);
}

/// Return a sorted copy using a custom key extraction function.
///
/// # Examples
///
/// ```
/// let data = vec![-5i32, -3, -1, 2, 4];
/// let sorted = tilesort::tilesorted_by_key(&data, |&x| x.abs());
/// assert_eq!(sorted, vec![-1, 2, -3, 4, -5]);
/// assert_eq!(data, vec![-5, -3, -1, 2, 4]); // Original unchanged
/// ```
pub fn tilesorted_by_key<T, K, F>(data: &[T], key_fn: F) -> Vec<T>
where
    T: Clone,
    K: Ord + Clone,
    F: Fn(&T) -> K,
{
    let mut result = data.to_vec();
    sorter::tilesort_impl_with_key(&mut result, key_fn, false);
    result
}

/// Return a sorted copy in descending order using a custom key extraction function.
///
/// # Examples
///
/// ```
/// let data = vec![-5i32, -3, -1, 2, 4];
/// let sorted = tilesort::tilesorted_by_key_reverse(&data, |&x| x.abs());
/// assert_eq!(sorted, vec![-5, 4, -3, 2, -1]);
/// assert_eq!(data, vec![-5, -3, -1, 2, 4]); // Original unchanged
/// ```
pub fn tilesorted_by_key_reverse<T, K, F>(data: &[T], key_fn: F) -> Vec<T>
where
    T: Clone,
    K: Ord + Clone,
    F: Fn(&T) -> K,
{
    let mut result = data.to_vec();
    sorter::tilesort_impl_with_key(&mut result, key_fn, true);
    result
}

// Python bindings (only when 'python' feature is enabled)
#[cfg(feature = "python")]
mod python_bindings {
    use pyo3::prelude::*;
    use pyo3::types::{PyAny, PyList};

    use crate::key_extractor::KeyExtractor;
    use crate::sorter::{tilesort_impl, tilesort_impl_with_key};
    use std::cmp::Ordering;

    /// Wrapper around PyObject that implements Ord using Python's comparison protocol
    struct PyOrd {
        obj: PyObject,
    }

    impl Clone for PyOrd {
        fn clone(&self) -> Self {
            Python::with_gil(|py| Self {
                obj: self.obj.clone_ref(py),
            })
        }
    }

    impl PyOrd {
        fn new(obj: PyObject) -> Self {
            Self { obj }
        }

        fn into_inner(self) -> PyObject {
            self.obj
        }
    }

    impl std::fmt::Debug for PyOrd {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Python::with_gil(|py| write!(f, "{:?}", self.obj.bind(py)))
        }
    }

    impl PartialEq for PyOrd {
        fn eq(&self, other: &Self) -> bool {
            Python::with_gil(|py| self.obj.bind(py).eq(other.obj.bind(py)).unwrap_or(false))
        }
    }

    impl Eq for PyOrd {}

    impl PartialOrd for PyOrd {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for PyOrd {
        fn cmp(&self, other: &Self) -> Ordering {
            Python::with_gil(|py| {
                let self_obj = self.obj.bind(py);
                let other_obj = other.obj.bind(py);

                if self_obj.lt(other_obj).unwrap_or(false) {
                    Ordering::Less
                } else if self_obj.gt(other_obj).unwrap_or(false) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
        }
    }

    /// Wrapper for Python callable key function
    struct PyKeyExtractor {
        key_fn: Py<PyAny>,
    }

    impl KeyExtractor<PyOrd, PyOrd> for PyKeyExtractor {
        fn extract_key(&self, item: &PyOrd) -> PyOrd {
            Python::with_gil(|py| {
                // Call the Python key function with the item
                let result = self
                    .key_fn
                    .call1(py, (&item.obj,))
                    .expect("Key function call failed");
                PyOrd::new(result)
            })
        }
    }

    /// Sort a Python list in place (like list.sort())
    ///
    /// # Arguments
    /// * `list` - The Python list to sort in place
    /// * `key` - Optional Python callable for key extraction
    /// * `reverse` - If true, sort in descending order
    #[pyfunction]
    #[pyo3(signature = (list, key=None, reverse=false))]
    fn sort(list: &Bound<'_, PyList>, key: Option<Py<PyAny>>, reverse: bool) -> PyResult<()> {
        Python::with_gil(|py| {
            // Extract Python objects from the list and wrap in PyOrd
            let mut items: Vec<PyOrd> = list.iter().map(|item| PyOrd::new(item.into())).collect();

            // Sort based on whether we have a key function
            if let Some(key_fn) = key {
                let extractor = PyKeyExtractor { key_fn };
                tilesort_impl_with_key(&mut items, extractor, reverse);
            } else {
                // Use Python's natural ordering via __lt__
                tilesort_impl(&mut items, reverse);
            }

            // Clear the original list and repopulate it
            let empty = PyList::empty(py);
            list.set_slice(0, list.len(), &empty)?;
            for item in items {
                list.append(item.into_inner())?;
            }

            Ok(())
        })
    }

    /// Return a sorted copy of a Python list (like sorted())
    ///
    /// # Arguments
    /// * `list` - The Python list to sort
    /// * `key` - Optional Python callable for key extraction
    /// * `reverse` - If true, sort in descending order
    #[pyfunction]
    #[pyo3(signature = (list, key=None, reverse=false))]
    fn sorted(
        list: &Bound<'_, PyList>,
        key: Option<Py<PyAny>>,
        reverse: bool,
    ) -> PyResult<Py<PyList>> {
        Python::with_gil(|py| {
            // Extract Python objects from the list and wrap in PyOrd
            let mut items: Vec<PyOrd> = list.iter().map(|item| PyOrd::new(item.into())).collect();

            // Sort based on whether we have a key function
            if let Some(key_fn) = key {
                let extractor = PyKeyExtractor { key_fn };
                tilesort_impl_with_key(&mut items, extractor, reverse);
            } else {
                // Use Python's natural ordering via __lt__
                tilesort_impl(&mut items, reverse);
            }

            // Create a new Python list with sorted items
            let unwrapped: Vec<PyObject> =
                items.into_iter().map(|item| item.into_inner()).collect();
            let new_list = PyList::new(py, unwrapped)?;
            Ok(new_list.into())
        })
    }

    /// A Python module implemented in Rust.
    #[pymodule]
    fn _tilesort(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(sort, m)?)?;
        m.add_function(wrap_pyfunction!(sorted, m)?)?;
        Ok(())
    }
}
