use tuirealm::application::PollStrategy;
// -- internal

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tui_realm_stdlib::utils::get_block;
use tui_realm_stdlib::Label;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, Borders, Color, Style, TextModifiers};
use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::Rect;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::Block;
use tuirealm::tui::widgets::{BorderType, Paragraph};
use tuirealm::{
    Application, AttrValue, Attribute, Component, Event, EventListenerCfg, Frame, MockComponent,
    NoUserEvent, Props, State, StateValue, Update,
};

struct Counter {
    props: Props,
    states: OwnStates,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            props: Props::default(),
            states: OwnStates::default(),
        }
    }
}
struct OwnStates {
    counter: isize,
}

impl Default for OwnStates {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

impl OwnStates {
    fn incr(&mut self) {
        self.counter += 1;
    }
}

impl Counter {
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Title,
            AttrValue::Title((label.as_ref().to_string(), Alignment::Center)),
        );
        self
    }

    pub fn value(mut self, n: isize) -> Self {
        self.attr(Attribute::Value, AttrValue::Number(n));
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }
}

impl MockComponent for Counter {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let text = self.states.counter.to_string();
            let alignment = self
                .props
                .get_or(Attribute::TextAlign, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let title = self
                .props
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            frame.render_widget(
                Paragraph::new(text)
                    .block(get_block(borders, Some(title), focus, None))
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(modifiers),
                    )
                    .alignment(alignment),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Isize(self.states.counter))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Submit => {
                self.states.incr();
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    DigitCounterChanged(isize),
    DigitCounterBlur,
    LetterCounterChanged(isize),
    LetterCounterBlur,
    /// Used to unwrap on update()
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    DigitCounter,
    LetterCounter,
    LabelValue,
    LabelValue2,
}

#[derive(MockComponent)]
pub struct LabelValue {
    component: Label,
}

impl Default for LabelValue {
    fn default() -> Self {
        Self {
            component: Label::default()
                .alignment(Alignment::Center)
                .foreground(Color::Green)
                .modifiers(TextModifiers::BOLD)
                .text("This is a label"),
        }
    }
}

impl Component<Msg, NoUserEvent> for LabelValue {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
pub struct LabelValue2 {
    component: Label,
}

impl Default for LabelValue2 {
    fn default() -> Self {
        Self {
            component: Label::default()
                .alignment(Alignment::Center)
                .foreground(Color::Green)
                .modifiers(TextModifiers::BOLD)
                .text("This is a label"),
        }
    }
}

impl Component<Msg, NoUserEvent> for LabelValue2 {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
pub struct LetterCounter {
    component: Counter,
}

impl LetterCounter {
    pub fn new(initial_value: isize) -> Self {
        Self {
            component: Counter::default()
                .alignment(Alignment::Center)
                .background(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD)
                .value(initial_value)
                .label("Letter counter"),
        }
    }
}

#[derive(MockComponent)]
pub struct DigitCounter {
    component: Counter,
}

impl DigitCounter {
    pub fn new(initial_value: isize) -> Self {
        Self {
            component: Counter::default()
                .alignment(Alignment::Center)
                .background(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD)
                .value(initial_value)
                .label("Letter counter"),
        }
    }
}

impl Component<Msg, NoUserEvent> for LetterCounter {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        // Get command
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) if ch.is_alphabetic() => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::LetterCounterBlur), // Return focus lost
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };
        // perform
        match self.perform(cmd) {
            CmdResult::Changed(State::One(StateValue::Isize(c))) => {
                Some(Msg::LetterCounterChanged(c))
            }
            _ => None,
        }
    }
}

impl Component<Msg, NoUserEvent> for DigitCounter {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        // Get command
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) if ch.is_digit(10) => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::LetterCounterBlur), // Return focus lost
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };
        // perform
        match self.perform(cmd) {
            CmdResult::Changed(State::One(StateValue::Isize(c))) => {
                Some(Msg::LetterCounterChanged(c))
            }
            _ => None,
        }
    }
}

pub struct Model {
    /// Application
    pub app: Application<Id, Msg, NoUserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge,
}

impl Model {
    pub fn view(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3), // Letter Counter
                            Constraint::Length(3), // Digit Counter
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                self.app.view(&Id::LetterCounter, f, chunks[0]);
                self.app.view(&Id::DigitCounter, f, chunks[1]);
            })
            .is_ok());
    }
    fn init_app() -> Application<Id, Msg, NoUserEvent> {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );
        assert!(app
            .mount(
                Id::LetterCounter,
                Box::new(LetterCounter::new(0)),
                Vec::default()
            )
            .is_ok());
        assert!(app
            .mount(
                Id::DigitCounter,
                Box::new(DigitCounter::new(5)),
                Vec::default()
            )
            .is_ok());

        assert!(app.active(&Id::LetterCounter).is_ok());
        app
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app(),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
        }
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            // Match message
            match msg {
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::DigitCounterBlur => {
                    // Give focus to letter counter
                    assert!(self.app.active(&Id::LetterCounter).is_ok());
                    None
                }
                Msg::DigitCounterChanged(v) => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::LabelValue,
                            Attribute::Text,
                            AttrValue::String(format!("DigitCounter has now value: {}", v))
                        )
                        .is_ok());
                    None
                }
                Msg::LetterCounterBlur => {
                    // Give focus to digit counter
                    assert!(self.app.active(&Id::DigitCounter).is_ok());
                    None
                }
                Msg::LetterCounterChanged(v) => {
                    // Update label
                    assert!(self
                        .app
                        .attr(
                            &Id::LabelValue2,
                            Attribute::Text,
                            AttrValue::String(format!("LetterCounter has now value: {}", v))
                        )
                        .is_ok());
                    None
                }
                Msg::None => None,
            }
        } else {
            None
        }
    }
}

fn main() {
    let mut model = Model::default();

    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();

    while !model.quit {
        // Tick
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                // Handle error...
            }
            Ok(messages) if messages.len() > 0 => {
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }
        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}
