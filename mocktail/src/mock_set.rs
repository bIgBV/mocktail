//! Mock set
use crate::{
    mock::Mock,
    mock_builder::{Then, When},
    request::Request,
};

/// A set of mocks.
#[derive(Default, Debug, Clone)]
pub struct MockSet(Vec<Mock>);

impl MockSet {
    /// Creates an empty mockset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of mocks.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Inserts a mock.
    pub fn insert(&mut self, mock: Mock) {
        if !self.contains(&mock) {
            self.0.push(mock);
            self.0.sort_by_key(|mock| mock.priority());
        }
    }

    // Returns `true` if the mockset contains the mock.
    pub fn contains(&self, mock: &Mock) -> bool {
        self.0.contains(mock)
    }

    /// Builds and inserts a mock with default options.
    pub fn mock<F>(&mut self, f: F)
    where
        F: FnOnce(When, Then),
    {
        let mock = Mock::new(f);
        self.insert(mock);
    }

    /// Builds and inserts a mock with options.
    pub fn mock_with_options<F>(&mut self, priority: u8, limit: Option<usize>, f: F)
    where
        F: FnOnce(When, Then),
    {
        let mut mock = Mock::new(f).with_priority(priority);
        if let Some(limit) = limit {
            mock = mock.with_limit(limit);
        }
        self.insert(mock);
    }

    /// Finds a mock by predicate.
    pub fn find<P>(&self, predicate: P) -> Option<&Mock>
    where
        P: FnMut(&&Mock) -> bool,
    {
        self.0.iter().find(predicate)
    }

    /// Removes a mock by index.
    pub fn remove(&mut self, index: usize) -> Mock {
        self.0.remove(index)
    }

    /// Clears the mockset.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns an iterator over the mockset.
    pub fn iter(&self) -> std::slice::Iter<'_, Mock> {
        self.0.iter()
    }

    /// Matches a request to a mock.
    pub fn match_by_request(&self, request: &Request) -> Option<Mock> {
        self.0.iter().find(|&mock| mock.matches(request)).cloned()
    }
}

impl IntoIterator for MockSet {
    type Item = Mock;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Mock> for MockSet {
    fn from_iter<I: IntoIterator<Item = Mock>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_builder() {
        let mut mocks = MockSet::new();
        mocks.mock(|when, then| {
            when.post().path("/hello").text("hello");
            then.text("hello!");
        });
        mocks.mock(|when, then| {
            when.post().path("/hello").text("hey");
            then.text("hello!");
        });
        assert_eq!(mocks.len(), 2);
    }
}
