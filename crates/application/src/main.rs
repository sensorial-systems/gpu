mod application;
pub use application::Application;

fn main() -> anyhow::Result<()> {
    Application::new()?.run()
}
