use super::Env;

pub struct LayeredEnv<T>(pub T);

macro_rules! impl_layered {
    ($generic0:tt: $tuple_field0:tt $(,$generic:tt: $tuple_field:tt)*) => {
        impl<$generic0:Env, $($generic: Env),*> Env for LayeredEnv<($generic0,$($generic,)*)> {
            fn get(&self, name: &str) -> Option<String> {
                self.0.$tuple_field0.get(name)
                    $(.or(self.0.$tuple_field.get(name)))*
            }
        }
    };
}

impl_layered!(A: 0);
impl_layered!(A: 0, B: 1);
impl_layered!(A: 0, B: 1, C: 2);
impl_layered!(A: 0, B: 1, C: 2, D: 3);
impl_layered!(A: 0, B: 1, C: 2, D: 3, E: 4);
impl_layered!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5);
impl_layered!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6);
impl_layered!(A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7);
