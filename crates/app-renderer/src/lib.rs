use futures::{channel::mpsc, stream::StreamExt};
use futures::prelude::*;
use wasm_bindgen::prelude::*;
use wee_alloc::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

fn register_sink<S: Sink<String>>(_sink: S) {
    // TODO: create closure and register sink with event handler
    unimplemented!()
}

async fn handle_stream<S: Stream + Unpin>(mut stream: S) -> Result<(), JsValue> {
    while let Some(_) = stream.next().await {
        // TODO: update buffer control
        // TODO: update tokens control
    }
    Ok(())
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let (sink, stream) = mpsc::unbounded::<String>();
    register_sink(sink);
    handle_stream(stream).await
}
