use crate::input::actions::Action;
use clap::ArgEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use strum_macros::{EnumDiscriminants, EnumIter, EnumString, ToString};

pub type ClientId = u16; // TODO: merge with crate type?

pub fn client_id_to_colors(
    client_id: ClientId,
    colors: Palette,
) -> Option<(PaletteColor, PaletteColor)> {
    // (primary color, secondary color)
    match client_id {
        1 => Some((colors.magenta, colors.black)),
        2 => Some((colors.blue, colors.black)),
        3 => Some((colors.purple, colors.black)),
        4 => Some((colors.yellow, colors.black)),
        5 => Some((colors.cyan, colors.black)),
        6 => Some((colors.gold, colors.black)),
        7 => Some((colors.red, colors.black)),
        8 => Some((colors.silver, colors.black)),
        9 => Some((colors.pink, colors.black)),
        10 => Some((colors.brown, colors.black)),
        _ => None,
    }
}

pub fn single_client_color(colors: Palette) -> (PaletteColor, PaletteColor) {
    (colors.green, colors.black)
}

// TODO: Add a shortened string representation (beyond `Display::fmt` below) that can be used when
// screen space is scarce. Useful for e.g. "ENTER", "SPACE", "TAB" to display as Unicode
// representations instead.
// NOTE: Do not reorder the key variants since that influences what the `status_bar` plugin
// displays!
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Key {
    PageDown,
    PageUp,
    Left,
    Down,
    Up,
    Right,
    Home,
    End,
    Backspace,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Alt(CharOrArrow),
    Ctrl(char),
    BackTab,
    Null,
    Esc,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Backspace => write!(f, "BACKSPACE"),
            Key::Left => write!(f, "{}", Direction::Left),
            Key::Right => write!(f, "{}", Direction::Right),
            Key::Up => write!(f, "{}", Direction::Up),
            Key::Down => write!(f, "{}", Direction::Down),
            Key::Home => write!(f, "HOME"),
            Key::End => write!(f, "END"),
            Key::PageUp => write!(f, "PgUp"),
            Key::PageDown => write!(f, "PgDn"),
            Key::BackTab => write!(f, "TAB"),
            Key::Delete => write!(f, "DEL"),
            Key::Insert => write!(f, "INS"),
            Key::F(n) => write!(f, "F{}", n),
            Key::Char(c) => match c {
                '\n' => write!(f, "ENTER"),
                '\t' => write!(f, "TAB"),
                ' ' => write!(f, "SPACE"),
                _ => write!(f, "{}", c),
            },
            Key::Alt(c) => write!(f, "Alt+{}", c),
            Key::Ctrl(c) => write!(f, "Ctrl+{}", Key::Char(*c)),
            Key::Null => write!(f, "NULL"),
            Key::Esc => write!(f, "ESC"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(untagged)]
pub enum CharOrArrow {
    Char(char),
    Direction(Direction),
}

impl fmt::Display for CharOrArrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CharOrArrow::Char(c) => write!(f, "{}", Key::Char(*c)),
            CharOrArrow::Direction(d) => write!(f, "{}", d),
        }
    }
}

/// The four directions (left, right, up, down).
#[derive(Eq, Clone, Copy, Debug, PartialEq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "←"),
            Direction::Right => write!(f, "→"),
            Direction::Up => write!(f, "↑"),
            Direction::Down => write!(f, "↓"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// FIXME: This should be extended to handle different button clicks (not just
// left click) and the `ScrollUp` and `ScrollDown` events could probably be
// merged into a single `Scroll(isize)` event.
pub enum Mouse {
    ScrollUp(usize),          // number of lines
    ScrollDown(usize),        // number of lines
    LeftClick(isize, usize),  // line and column
    RightClick(isize, usize), // line and column
    Hold(isize, usize),       // line and column
    Release(isize, usize),    // line and column
}

#[derive(Debug, Clone, PartialEq, EnumDiscriminants, ToString, Serialize, Deserialize)]
#[strum_discriminants(derive(EnumString, Hash, Serialize, Deserialize))]
#[strum_discriminants(name(EventType))]
#[non_exhaustive]
pub enum Event {
    ModeUpdate(ModeInfo),
    TabUpdate(Vec<TabInfo>),
    Key(Key),
    Mouse(Mouse),
    Timer(f64),
    CopyToClipboard(CopyDestination),
    SystemClipboardFailure,
    InputReceived,
    Visible(bool),
}

/// Describes the different input modes, which change the way that keystrokes will be interpreted.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter, Serialize, Deserialize, ArgEnum)]
pub enum InputMode {
    /// In `Normal` mode, input is always written to the terminal, except for the shortcuts leading
    /// to other modes
    #[serde(alias = "normal")]
    Normal,
    /// In `Locked` mode, input is always written to the terminal and all shortcuts are disabled
    /// except the one leading back to normal mode
    #[serde(alias = "locked")]
    Locked,
    /// `Resize` mode allows resizing the different existing panes.
    #[serde(alias = "resize")]
    Resize,
    /// `Pane` mode allows creating and closing panes, as well as moving between them.
    #[serde(alias = "pane")]
    Pane,
    /// `Tab` mode allows creating and closing tabs, as well as moving between them.
    #[serde(alias = "tab")]
    Tab,
    /// `Scroll` mode allows scrolling up and down within a pane.
    #[serde(alias = "scroll")]
    Scroll,
    /// `EnterSearch` mode allows for typing in the needle for a search in the scroll buffer of a pane.
    #[serde(alias = "entersearch")]
    EnterSearch,
    /// `Search` mode allows for searching a term in a pane (superset of `Scroll`).
    #[serde(alias = "search")]
    Search,
    /// `RenameTab` mode allows assigning a new name to a tab.
    #[serde(alias = "renametab")]
    RenameTab,
    /// `RenamePane` mode allows assigning a new name to a pane.
    #[serde(alias = "renamepane")]
    RenamePane,
    /// `Session` mode allows detaching sessions
    #[serde(alias = "session")]
    Session,
    /// `Move` mode allows moving the different existing panes within a tab
    #[serde(alias = "move")]
    Move,
    /// `Prompt` mode allows interacting with active prompts.
    #[serde(alias = "prompt")]
    Prompt,
    /// `Tmux` mode allows for basic tmux keybindings functionality
    #[serde(alias = "tmux")]
    Tmux,
}

impl Default for InputMode {
    fn default() -> InputMode {
        InputMode::Normal
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ThemeHue {
    Light,
    Dark,
}
impl Default for ThemeHue {
    fn default() -> ThemeHue {
        ThemeHue::Dark
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaletteColor {
    Rgb((u8, u8, u8)),
    EightBit(u8),
}
impl Default for PaletteColor {
    fn default() -> PaletteColor {
        PaletteColor::EightBit(0)
    }
}

impl FromStr for InputMode {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(InputMode::Normal),
            "resize" => Ok(InputMode::Resize),
            "locked" => Ok(InputMode::Locked),
            "pane" => Ok(InputMode::Pane),
            "tab" => Ok(InputMode::Tab),
            "scroll" => Ok(InputMode::Scroll),
            "search" => Ok(InputMode::Search),
            "entersearch" => Ok(InputMode::EnterSearch),
            "renametab" => Ok(InputMode::RenameTab),
            "session" => Ok(InputMode::Session),
            "move" => Ok(InputMode::Move),
            "tmux" => Ok(InputMode::Tmux),
            "prompt" => Ok(InputMode::Prompt),
            "renamepane" => Ok(InputMode::RenamePane),
            e => Err(e.to_string().into()),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PaletteSource {
    Default,
    Xresources,
}
impl Default for PaletteSource {
    fn default() -> PaletteSource {
        PaletteSource::Default
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct Palette {
    pub source: PaletteSource,
    pub theme_hue: ThemeHue,
    pub fg: PaletteColor,
    pub bg: PaletteColor,
    pub black: PaletteColor,
    pub red: PaletteColor,
    pub green: PaletteColor,
    pub yellow: PaletteColor,
    pub blue: PaletteColor,
    pub magenta: PaletteColor,
    pub cyan: PaletteColor,
    pub white: PaletteColor,
    pub orange: PaletteColor,
    pub gray: PaletteColor,
    pub purple: PaletteColor,
    pub gold: PaletteColor,
    pub silver: PaletteColor,
    pub pink: PaletteColor,
    pub brown: PaletteColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Style {
    pub colors: Palette,
    pub rounded_corners: bool,
}

// FIXME: Poor devs hashtable since HashTable can't derive `Default`...
pub type KeybindsVec = Vec<(InputMode, Vec<(Key, Vec<Action>)>)>;

/// Represents the contents of the help message that is printed in the status bar,
/// which indicates the current [`InputMode`] and what the keybinds for that mode
/// are. Related to the default `status-bar` plugin.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeInfo {
    pub mode: InputMode,
    pub keybinds: KeybindsVec,
    pub style: Style,
    pub capabilities: PluginCapabilities,
    pub session_name: Option<String>,
}

impl ModeInfo {
    pub fn get_mode_keybinds(&self) -> Vec<(Key, Vec<Action>)> {
        self.get_keybinds_for_mode(self.mode)
    }

    pub fn get_keybinds_for_mode(&self, mode: InputMode) -> Vec<(Key, Vec<Action>)> {
        for (vec_mode, map) in &self.keybinds {
            if mode == *vec_mode {
                return map.to_vec();
            }
        }
        vec![]
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TabInfo {
    /* subset of fields to publish to plugins */
    pub position: usize,
    pub name: String,
    pub active: bool,
    pub panes_to_hide: usize,
    pub is_fullscreen_active: bool,
    pub is_sync_panes_active: bool,
    pub are_floating_panes_visible: bool,
    pub other_focused_clients: Vec<ClientId>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PluginIds {
    pub plugin_id: u32,
    pub zellij_pid: u32,
}

/// Tag used to identify the plugin in layout and config yaml files
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PluginTag(String);

impl PluginTag {
    pub fn new(url: impl Into<String>) -> Self {
        PluginTag(url.into())
    }
}

impl From<PluginTag> for String {
    fn from(tag: PluginTag) -> Self {
        tag.0
    }
}

impl fmt::Display for PluginTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PluginCapabilities {
    pub arrow_fonts: bool,
}

impl Default for PluginCapabilities {
    fn default() -> PluginCapabilities {
        PluginCapabilities { arrow_fonts: true }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CopyDestination {
    Command,
    Primary,
    System,
}
