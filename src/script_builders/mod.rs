pub mod app_script;
pub mod depot_script;

pub trait InteractiveNew {
    fn interactive_new(id: String) -> Self;
}