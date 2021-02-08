use alloc::rc::Rc;
use bitfield::{bitfield, Bit};
use core::cell::RefCell;

use super::hal::Joypad as Button;
use super::hal::HAL;

#[derive(Eq, PartialEq)]
pub enum SelectState {
    Selected,
    NotSelected,
}

impl From<u8> for SelectState {
    fn from(value: u8) -> Self {
        match value {
            0 => SelectState::Selected,
            1 => SelectState::NotSelected,
            _ => unreachable!(),
        }
    }
}

impl Into<bool> for SelectState {
    fn into(self) -> bool {
        match self {
            SelectState::Selected => true,
            SelectState::NotSelected => false,
        }
    }
}

impl Into<u8> for SelectState {
    fn into(self) -> u8 {
        match self {
            SelectState::Selected => 0,
            SelectState::NotSelected => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl From<u8> for ButtonState {
    fn from(value: u8) -> Self {
        match value {
            0 => ButtonState::Pressed,
            1 => ButtonState::Released,
            _ => unreachable!(),
        }
    }
}

impl std::ops::BitOr for ButtonState {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self {
        if self == ButtonState::Pressed || rhs == ButtonState::Pressed {
            ButtonState::Pressed
        } else {
            ButtonState::Released
        }
    }
}

impl Into<u8> for ButtonState {
    fn into(self) -> u8 {
        match self {
            ButtonState::Pressed => 0,
            ButtonState::Released => 1,
        }
    }
}

impl Into<bool> for ButtonState {
    fn into(self) -> bool {
        match self {
            ButtonState::Pressed => true,
            ButtonState::Released => false,
        }
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct JOYP(u8);
    //impl Debug;
    u8;
    pub from into SelectState, select_buttons, set_select_buttons: 5, 5;
    pub from into SelectState, select_directions, set_select_directions: 4, 4;
    pub inputs, set_inputs: 3, 0;
    pub from into ButtonState, down_or_start, set_down_or_start: 3, 3;
    pub from into ButtonState, up_or_select, set_up_or_select: 2, 2;
    pub from into ButtonState, left_or_b, set_left_or_b: 1, 1;
    pub from into ButtonState, right_or_a, set_right_or_a: 0, 0;
}

impl Into<u8> for JOYP {
    fn into(self) -> u8 {
        self.0
    }
}

#[derive(Clone)]
pub struct Joypad {
    joyp: JOYP,

    hal: Rc<RefCell<dyn HAL>>,
}

impl Joypad {
    pub fn new(hal: Rc<RefCell<dyn HAL>>) -> Self {
        Joypad {
            joyp: JOYP(0x0F),

            hal,
        }
    }

    pub fn joyp(&self) -> JOYP {
        //self.joyp.clone()
        let hal = self.hal.borrow();

        let get_button = |button| {
            if hal.is_joypad_pressed(button) {
                ButtonState::Pressed
            } else {
                ButtonState::Released
            }
        };

        let mut directions = JOYP(0x0F);

        if let SelectState::Selected = self.joyp.select_directions() {
            directions.set_down_or_start(get_button(Button::Down));
            directions.set_up_or_select(get_button(Button::Up));
            directions.set_left_or_b(get_button(Button::Left));
            directions.set_right_or_a(get_button(Button::Right));
        }

        let mut buttons = JOYP(0x0F);

        if let SelectState::Selected = self.joyp.select_buttons() {
            buttons.set_down_or_start(get_button(Button::Start));
            buttons.set_up_or_select(get_button(Button::Select));
            buttons.set_left_or_b(get_button(Button::B));
            buttons.set_right_or_a(get_button(Button::A));
        }

        let mut joyp = self.joyp.clone();
        joyp.set_down_or_start(directions.down_or_start() | buttons.down_or_start());
        joyp.set_up_or_select(directions.up_or_select() | buttons.up_or_select());
        joyp.set_left_or_b(directions.left_or_b() | buttons.left_or_b());
        joyp.set_right_or_a(directions.right_or_a() | buttons.right_or_a());

        joyp
    }

    pub fn set_joyp(&mut self, value: u8) {
        self.joyp.set_select_buttons(if !value.bit(5) {
            SelectState::Selected
        } else {
            SelectState::NotSelected
        });

        self.joyp.set_select_directions(if !value.bit(4) {
            SelectState::Selected
        } else {
            SelectState::NotSelected
        })
    }

    fn should_interrupt(&mut self, previous: &Joypad) -> bool {
        (0..4)
            .into_iter()
            .any(|bit| self.joyp.bit(bit) && !previous.joyp.bit(bit))
    }

    pub fn tick_m_cycle(&mut self) -> bool {
        //let previous = self.clone();
        //self.should_interrupt(&previous)

        // TODO: This
        false
    }
}
