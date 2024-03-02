use std::io;
use ratatui::{layout::Rect, Frame};

use crate::action::Action;

pub mod url;
pub mod submit;
pub mod response;
pub mod headers;
pub mod history;

/// `Component` is a trait that represents a visual and interactive element of the user interface.
/// Implementors of this trait can be registered with the main application loop and will be able to receive events,
/// update state, and be rendered on the screen.
pub trait Component {

  fn handle_deselect(&mut self) -> Option<Action>;


  /// Handle selection the component (REQUIRED)
  ///
  fn handle_select(&mut self);

  /// Handle key events
  ///
  /// # Arguments
  ///
  /// * `key` - A key event to be processed.
  ///
  /// # Returns
  ///
  /// * `Result<Option<Action>>` - An action to be processed or none.
  fn handle_key_events(&mut self) -> Option<Action>;

  /// Render the component on the screen. (REQUIRED)
  ///
  /// # Arguments
  ///
  /// * `f` - A frame used for rendering.
  /// * `area` - The area in which the component should be drawn.
  ///
  /// # Returns
  ///
  /// * `Result<()>` - An Ok result or an error.
  fn render_frame(&mut self, frame: &mut Frame<'_>, area: Rect) -> io::Result<()>;
}
