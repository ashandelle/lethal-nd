use crate::client::InputSettings;

#[derive(Debug)]
pub struct ClientSettings<const N: usize> where [(); N - 1]: Sized {
    pub inputsettings: InputSettings<N>,
}

impl<const N: usize> Default for ClientSettings<N> where InputSettings<N>: Default, [(); N - 1]: Sized {
    fn default() -> Self {
        ClientSettings {
            inputsettings: Default::default(),
        }
    }
}