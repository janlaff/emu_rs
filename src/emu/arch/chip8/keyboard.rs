pub struct Keyboard {
    pub state: [bool; 16],
    pub wait_for_key: bool,
    pub key_received: bool,
    pub key: u8,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            state: [false; 16],
            wait_for_key: false,
            key_received: false,
            key: 0,
        }
    }

    pub fn press_key(&mut self, key: u8) {
        self.state[key as usize] = true;

        if self.wait_for_key {
            self.key = key;
            self.key_received = true;
        }
    }

    pub fn release_key(&mut self, key: u8) {
        self.state[key as usize] = false;
    }
}