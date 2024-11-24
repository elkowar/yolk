use sysinfo::SystemInfo;

pub mod eval_ctx;
pub mod stdlib;
pub mod sysinfo;

pub fn make_engine() -> rhai::Engine {
    let mut engine = rhai::Engine::new();
    engine
        .register_type::<SystemInfo>()
        .build_type::<SystemInfo>();

    stdlib::register(&mut engine);
    engine
}
