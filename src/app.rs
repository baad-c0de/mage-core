use std::time::Duration;

/// The [`App`] trait is the main interface for the game. It is called by the
/// framework to update the game state and render the game.
///
/// Mage manages a window and GPU state to render ASCII characters to the
/// screen.  The [`App`] trait is used by the host to hook into the engine's
/// main loop.
///
/// The [`tick`] method is called once per frame to update the game state. The
/// [`present`] method is called once per frame to render the game.
///
/// # Tick method
///
/// The [`tick_input`] parameter contains information about the current frame to
/// be helpful with updating the game state. This includes the time since the
/// last frame, the size of the window (in characters), and the current state of
/// the keyboard and mouse.
///
/// It returns a [`TickResult`] to indicate whether the game should continue or
/// quit.
///
/// # Present method
///
/// The [`present_input`] parameter contains information about the current frame
/// to be helpful with rendering the game. This includes the size of the window
/// (in characters), and the current state of the screen.
///
/// The screen is represented as 3 32-bit RGBA buffers, one for the foreground
/// color of each character, one for the background color of each character, and
/// one for each character.
///
/// For the foreground and background colors, each 32-bit RGBA value represents
/// a single character.  The least significant 8 bits are the red value, the
/// next 8 bits are the green value, the next 8 bits are the blue value, and the
/// most significant 8 bits are the alpha value. The alpha value is unused by
/// the engine but is available for use by the game.
///
/// For the character buffer, each 32-bit RGBA value represents a single
/// character.  The least significant 8 bits are the character value, the most
/// significant 24 bits are unused by the engine but are available for use by
/// the game.
///
/// It returns a [`PresentResult`] to indicate whether the screen has changed
/// since the last frame.
///
/// [`App`]: trait.App.html
/// [`tick`]: trait.App.html#tymethod.tick
/// [`present`]: trait.App.html#tymethod.present
/// [`tick_input`]: struct.TickInput.html
/// [`TickResult`]: enum.TickResult.html
/// [`present_input`]: struct.PresentInput.html
/// [`PresentResult`]: enum.PresentResult.html
///
pub trait App {
    /// Called once per frame to update the game state.
    ///
    /// # Parameters
    ///
    /// * `tick_input` - Information about the current frame.
    ///
    /// # Returns
    ///
    /// A [`TickResult`] to indicate whether the game should continue or quit.
    ///
    /// [`TickResult`]: enum.TickResult.html
    ///
    fn tick(&mut self, tick_input: TickInput) -> TickResult;

    /// Called once per frame to render the game.
    ///
    /// # Parameters
    ///
    /// * `present_input` - Information about the current frame.
    ///
    /// # Returns
    ///
    /// A [`PresentResult`] to indicate whether the screen has changed since the
    /// last frame.
    ///
    /// [`PresentResult`]: enum.PresentResult.html
    ///
    fn present(&mut self, present_input: PresentInput) -> PresentResult;
}

/// The [`TickResult`] is returned by the [`tick`] method of the [`App`] trait
/// to indicate whether the game should continue or quit.
///
/// [`TickResult`]: enum.TickResult.html
/// [`tick`]: trait.App.html#tymethod.tick
/// [`App`]: trait.App.html
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TickResult {
    /// Indicates the game should continue.
    Continue,

    /// Indicates the game should quit.
    Quit,
}

/// The [`PresentResult`] is returned by the [`present`] method of the [`App`]
/// trait to indicate whether the screen has changed since the last frame.
///
/// [`PresentResult`]: enum.PresentResult.html
/// [`present`]: trait.App.html#tymethod.present
/// [`App`]: trait.App.html
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PresentResult {
    /// Indicates the screen has changed since the last frame.
    Changed,

    /// Indicates the screen has not changed since the last frame.
    NoChanges,
}

/// The [`TickInput`] struct is passed to the [`tick`] method of the [`App`]
/// trait to provide information about the current frame.
///
/// [`TickInput`]: struct.TickInput.html
/// [`tick`]: trait.App.html#tymethod.tick
/// [`App`]: trait.App.html
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TickInput {
    /// The time since the last frame.
    pub dt: Duration,

    /// The width of the window in characters.
    pub width: u32,

    /// The height of the window in characters.
    pub height: u32,
}

/// The [`PresentInput`] struct is passed to the [`present`] method of the
/// [`App`] trait to provide information about the current frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentInput<'textures> {
    /// The width of the window in characters.
    pub width: u32,

    /// The height of the window in characters.
    pub height: u32,

    /// The foreground color of each character.  Each 32-bit RGBA value
    /// represents a single character.  The least significant 8 bits are the
    /// red value, the next 8 bits are the green value, the next 8 bits are the
    /// blue value, and the most significant 8 bits are the alpha value. The
    /// alpha value is currently unused.
    pub fore_image: &'textures Vec<u32>,

    /// The background color of each character.  Each 32-bit RGBA value
    /// represents a single character.  The least significant 8 bits are the
    /// red value, the next 8 bits are the green value, the next 8 bits are the
    /// blue value, and the most significant 8 bits are the alpha value. The
    /// alpha value is currently unused.
    pub back_image: &'textures Vec<u32>,

    /// The character buffer.  Each 32-bit RGBA value represents a single
    /// character. The least significant 8 bits are the ASCII value of the
    /// character, and the most significant 24 bits are unused by the engine but
    /// are available for use by the game.
    pub text_image: &'textures Vec<u32>,
}
