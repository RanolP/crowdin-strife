pub mod commands;
pub mod e2k_base;
pub mod file_reader;

#[cfg(target_arch = "wasm32")]
mod worker_app;

#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
#[worker::event(fetch)]
pub async fn main(
    req: worker::Request,
    env: worker::Env,
    ctx: worker::Context,
) -> worker::Result<worker::Response> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    worker_app::actual_main(req, env, ctx).await
}
