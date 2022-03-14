impl super::Error {
    pub(crate) fn is_interrupted_impl(&self) -> bool {
        false
    }

    pub(crate) fn from_kind_impl(kind: super::ErrorKind) -> Self {
        super::Error {
            inner: super::ErrorInner::Kind(kind),
        }
    }
}

