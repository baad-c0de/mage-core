use winit::keyboard::ModifiersState;

pub struct ShiftState {
    shift: bool,
    ctrl: bool,
    alt: bool,
}

impl ShiftState {
    pub fn new() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
        }
    }

    pub fn shift_down(&self) -> bool {
        self.shift
    }

    pub fn ctrl_down(&self) -> bool {
        self.ctrl
    }

    pub fn alt_down(&self) -> bool {
        self.alt
    }

    pub fn shift_only(&self) -> bool {
        self.shift && !self.ctrl && !self.alt
    }

    pub fn ctrl_only(&self) -> bool {
        !self.shift && self.ctrl && !self.alt
    }

    pub fn alt_only(&self) -> bool {
        !self.shift && !self.ctrl && self.alt
    }

    pub fn shift_ctrl(&self) -> bool {
        self.shift && self.ctrl && !self.alt
    }

    pub fn shift_alt(&self) -> bool {
        self.shift && !self.ctrl && self.alt
    }

    pub fn ctrl_alt(&self) -> bool {
        !self.shift && self.ctrl && self.alt
    }

    pub fn shift_ctrl_alt(&self) -> bool {
        self.shift && self.ctrl && self.alt
    }

    pub fn update(&mut self, modifiers: ModifiersState) {
        self.shift = modifiers.shift_key();
        self.ctrl = modifiers.control_key();
        self.alt = modifiers.alt_key();
    }
}

impl Default for ShiftState {
    fn default() -> Self {
        Self::new()
    }
}
