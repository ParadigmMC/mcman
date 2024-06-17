# Technical Documentation

meow

Everything starts at the [`App`](./src/api/app/mod.rs) struct.

- `App::collect_sources(&self) -> Vec<Source>`
- `Source::resolve_addons(&App) -> Vec<Addon>`
- `App::collect_addons(&self) -> Vec<Addon>`
- `Addon::resolve_steps(&App) -> Vec<Step>`
- `App::execute_steps(&self, &[Step]) -> Result<()>`
- `App::execute_step(&self, Step) -> Result<StepResult>`
