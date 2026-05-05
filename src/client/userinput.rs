use macroquad::{input::{KeyCode, MouseButton, is_key_down, is_mouse_button_down, mouse_delta_position, mouse_position, mouse_wheel}, prelude::camera::mouse};

#[derive(Debug)]
pub struct InputSettings<const N: usize> where [(); N - 1]: Sized {
    pub lookspeed: f32,
    pub movementkeys: [(KeyCode, KeyCode); N-1], // Movement keys (+, -) corresponding to each dimension [1,..,N-1], not including 0 (up/down)
    pub defaultlook: ((usize, usize), (usize, usize), Option<(usize, usize)>, Option<(usize, usize)>), // Dimensions to rotate in when moving mouse x, mouse y, scroll wheel up/down, scroll wheel left/right
    pub altlook: Vec<(KeyMouseBind, (usize, usize), (usize, usize), Option<(usize, usize)>, Option<(usize, usize)>)>, // Alternative look modes
    pub menukey: KeyCode,
}

#[derive(Debug)]
pub struct UserInput<const N: usize> where [(); N - 1]: Sized {
    pub movementkeys: [(bool, bool); N-1],
    pub look: ((usize, usize, f32), (usize, usize, f32), Option<(usize, usize, f32)>, Option<(usize, usize, f32)>),
    pub menu: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum KeyMouseBind {
    Key { keycode: KeyCode },
    Mouse { mousebutton: MouseButton },
}

pub fn is_keymouse_down(keymouse: KeyMouseBind) -> bool {
    match keymouse {
        KeyMouseBind::Key { keycode } => {
            is_key_down(keycode)
        },
        KeyMouseBind::Mouse { mousebutton } => {
            is_mouse_button_down(mousebutton)
        },
    }
}

impl<const N: usize> InputSettings<N> where [(); N - 1]: Sized {
    pub fn get_input(&self) -> UserInput<N> {
        let movementkeys: [(bool, bool); N-1] = std::array::from_fn(|i| {
            let (plus, minus) = self.movementkeys[i];
            (is_key_down(plus), is_key_down(minus))
        });

        let mousepos = mouse_delta_position();
        let mousepos = (mousepos.x, mousepos.y);
        let mousewheel = mouse_wheel();

        let mut look = (
            (self.defaultlook.0.0, self.defaultlook.0.1, mousepos.0 * self.lookspeed),
            (self.defaultlook.1.0, self.defaultlook.1.1, mousepos.1 * self.lookspeed),
            match self.defaultlook.2 {
                Some((a, b)) => Some((a, b, mousewheel.1 * self.lookspeed)),
                None => None,
            },
            match self.defaultlook.3 {
                Some((a, b)) => Some((a, b, mousewheel.0 * self.lookspeed)),
                None => None,
            }
        );

        for (keymouse, mousex, mousey, scrolly, scrollx) in &self.altlook {
            if is_keymouse_down(*keymouse) {
                look = (
                    (mousex.0, mousex.1, mousepos.0 * self.lookspeed),
                    (mousey.0, mousey.1, mousepos.1 * self.lookspeed),
                    match scrolly {
                        Some((a, b)) => Some((*a, *b, mousewheel.1 * self.lookspeed)),
                        None => None,
                    },
                    match scrollx {
                        Some((a, b)) => Some((*a, *b, mousewheel.0 * self.lookspeed)),
                        None => None,
                    }
                );
                break;
            }
        }

        UserInput {
            movementkeys,
            look,
            menu: is_key_down(self.menukey),
        }
    }
}

impl Default for InputSettings<3> {
    fn default() -> Self {
        Self {
            lookspeed: 5.0,
            movementkeys: [
                (KeyCode::W, KeyCode::S),
                (KeyCode::D, KeyCode::A)
            ],
            defaultlook: ((2, 1), (1, 0), None, None),
            altlook: Vec::new(),
            menukey: KeyCode::Escape,
        }
    }
}

impl Default for InputSettings<4> {
    fn default() -> Self {
        Self {
            lookspeed: 5.0,
            movementkeys: [
                (KeyCode::W, KeyCode::S),
                (KeyCode::D, KeyCode::A),
                (KeyCode::E, KeyCode::Q)
            ],
            defaultlook: ((2, 1), (1, 0), None, None),
            altlook: vec![
                (KeyMouseBind::Mouse { mousebutton: MouseButton::Middle }, (2, 3), (1, 3), None, None)
            ],
            menukey: KeyCode::Escape,
        }
    }
}