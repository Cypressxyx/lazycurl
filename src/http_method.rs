use ratatui::text::Line;
use strum::{Display, EnumIter, FromRepr};

#[derive(Display, FromRepr, Clone, Copy, EnumIter)]
pub enum HTTPMethod {
    POST,
    GET,
}

impl HTTPMethod {
    pub fn line(self) -> Line<'static> {
        format!(" {self} ")
            .into()
    }

    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn prev(self) -> Self {
        let current_index = self as usize;
        let prev_index = current_index.saturating_sub(1);
        Self::from_repr(prev_index).unwrap_or(self)
    }
}
