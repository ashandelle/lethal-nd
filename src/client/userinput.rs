use macroquad::input::{KeyCode, is_key_down};

#[derive(Debug)]
pub struct InputSettings<const N: usize> where [(); N - 1]: Sized {
    pub movementkeys: [(KeyCode, KeyCode); N-1], // Movement keys (+, -) corresponding to each dimension [1,..,N-1], not including 0 (up/down)
}

#[derive(Debug)]
pub struct UserInput<const N: usize> where [(); N - 1]: Sized {
    pub movementkeys: [(bool, bool); N-1],
}

impl<const N: usize> InputSettings<N> where [(); N - 1]: Sized {
    pub fn get_input(&self) -> UserInput<N> {
        let movementkeys: [(bool, bool); N-1] = std::array::from_fn(|i| {
            let (plus, minus) = self.movementkeys[i];
            (is_key_down(plus), is_key_down(minus))
        });
        UserInput {
            movementkeys
        }
    }
}

impl Default for InputSettings<3> {
    fn default() -> Self {
        Self {
            movementkeys: [
                (KeyCode::W, KeyCode::S),
                (KeyCode::D, KeyCode::A)
            ],
        }
    }
}

impl Default for InputSettings<4> {
    fn default() -> Self {
        Self {
            movementkeys: [
                (KeyCode::W, KeyCode::S),
                (KeyCode::D, KeyCode::A),
                (KeyCode::E, KeyCode::Q)
            ],
        }
    }
}