use neon::prelude::*;
use once_cell::sync::OnceCell;
use reqwest;
use std::time::Duration;
use feed_rs::parser;
use reqwest::{Client};
use tokio::runtime::Runtime;

// import feed builder / manual serializer
mod feed_builder;
use crate::feed_builder::feed_builder::{from};

// Return a global tokio runtime or create one if it doesn't exist.
// Throws a JavaScript exception if the `Runtime` fails to create.
fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

// fetch feed
async fn fetch_feed(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    client.get(url)
        .header("Accept", "text/plain,application/xml")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await
}

// get url feed process, parse and serialize into json
// convert types into node types and "Send" promise to node event-loop thread with deferred
fn from_url(mut cx: FunctionContext) -> JsResult<JsPromise> {

    let url_handle = cx.argument::<JsString>(0)?;
    let url = url_handle.value(&mut cx);
    let rt = runtime(&mut cx)?;
    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    rt.spawn(async move {

        let fetch_result = fetch_feed(&url).await;

        match fetch_result {

            Err(e) => Err(e),
            Ok(feed_text) => Ok({

                let parse_result = parser::parse(feed_text.as_bytes()).unwrap();
                let serialize_result = from(&parse_result);
                let json = serde_json::to_string_pretty(&serialize_result).unwrap();

                deferred.settle_with(&channel, move |mut cx| {
                    Ok(cx.string(json))
                });
            })
        }
    });

    Ok(promise)
}

// test function for sync
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

// define main exports to node compiled binary
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("fromUrl", from_url)?;
    cx.export_function("hello", hello)?;
    Ok(())
}
