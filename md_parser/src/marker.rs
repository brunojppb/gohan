use std::ops::Range;

#[derive(Debug)]
pub struct LinkMarker {
    pub start_text: Option<usize>,
    pub end_text: Option<usize>,
    pub start_url: Option<usize>,
    pub end_url: Option<usize>,
}

/// helful for holding the boundaries of a Link element during parsing
impl LinkMarker {
    pub fn new() -> Self {
        Self {
            start_text: None,
            end_text: None,
            start_url: None,
            end_url: None,
        }
    }

    pub fn set_start_text(&mut self, index: usize) {
        self.start_text = Some(index);
    }

    pub fn set_end_text(&mut self, index: usize) {
        self.end_text = Some(index);
    }

    pub fn set_start_url(&mut self, index: usize) {
        self.start_url = Some(index);
    }

    pub fn set_end_url(&mut self, index: usize) {
        self.end_url = Some(index);
    }

    pub fn is_link(&self) -> bool {
        self.start_text.is_some()
            && self.end_text.is_some()
            && self.start_url.is_some()
            && self.end_url.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.start_text.is_none()
            && self.end_text.is_none()
            && self.start_url.is_none()
            && self.end_url.is_none()
    }

    pub fn has_open_text(&self) -> bool {
        self.start_text.is_some()
            && self.end_text.is_none()
            && self.start_url.is_none()
            && self.end_url.is_none()
    }

    pub fn has_open_url(&self) -> bool {
        self.start_text.is_some()
            && self.end_text.is_some()
            && self.start_url.is_some()
            && self.end_url.is_none()
    }

    /// given a complete link, extract the ranges of its inner components
    /// Tuple composed by ranges of: (link text, link URL)
    pub fn ranges(&self) -> Option<(Range<usize>, Range<usize>)> {
        match (self.start_text, self.end_text, self.start_url, self.end_url) {
            (Some(text_start), Some(text_end), Some(url_start), Some(url_end)) => {
                Some((text_start..text_end, url_start..url_end))
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct InlineMarker {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

impl InlineMarker {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    pub fn is_open(&self) -> bool {
        self.start.is_some() && self.end.is_none()
    }

    pub fn is_closed(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    pub fn open(&mut self, index: usize) {
        self.start = Some(index);
    }

    pub fn close(&mut self, index: usize) {
        self.end = Some(index);
    }

    pub fn range(&self) -> Option<Range<usize>> {
        match (self.start, self.end) {
            (Some(start), Some(end)) => {
                let range = start..end;
                if range.is_empty() {
                    None
                } else {
                    Some(range)
                }
            }
            _ => None,
        }
    }
}
