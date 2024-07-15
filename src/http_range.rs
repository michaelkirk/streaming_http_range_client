use core::ops::{Range, RangeFrom};

#[derive(Debug, Clone)]
pub enum HttpRange {
    Range(Range<u64>),
    RangeFrom(RangeFrom<u64>),
}

impl HttpRange {
    pub fn start(&self) -> u64 {
        match self {
            Self::Range(range) => range.start,
            Self::RangeFrom(range) => range.start,
        }
    }

    pub fn end(&self) -> Option<u64> {
        match self {
            Self::Range(range) => Some(range.end),
            Self::RangeFrom(_) => None,
        }
    }

    pub fn with_end(self, end: Option<u64>) -> Self {
        match end {
            Some(end) => Self::Range(self.start()..end),
            None => Self::RangeFrom(self.start()..),
        }
    }

    pub fn split(&mut self, new_end: u64) -> Self {
        assert!(
            new_end > self.start(),
            "new_end is before where we already start (rewinding?) new_end: {}, start: {}",
            new_end,
            self.start()
        );
        match self {
            Self::Range(range) => {
                assert!(new_end <= range.end);
                let old_end = range.end;
                range.end = new_end;
                Self::Range(new_end..old_end)
            }
            Self::RangeFrom(range) => {
                *self = Self::Range(range.start..new_end);
                Self::RangeFrom(new_end..)
            }
        }
    }
}

impl From<Range<u64>> for HttpRange {
    fn from(value: Range<u64>) -> Self {
        Self::Range(value)
    }
}

impl From<RangeFrom<u64>> for HttpRange {
    fn from(value: RangeFrom<u64>) -> Self {
        Self::RangeFrom(value)
    }
}
