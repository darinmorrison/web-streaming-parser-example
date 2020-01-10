use futures::{channel::mpsc, prelude::*, stream::StreamExt};
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{alpha1, alphanumeric0, multispace0},
    combinator::recognize,
    sequence::tuple,
    IResult,
};
use nom_async::NomStream;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, HtmlInputElement, HtmlOptionElement, HtmlSelectElement, HtmlTextAreaElement, KeyboardEvent};
use wee_alloc::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Token {
    Lpar,
    Rpar,
    Sym(String),
}

pub fn token<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let (i, _) = multispace0(input)?;
    alt((lpar, rpar, sym))(i)
}

pub fn lpar<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let (i, _) = tag("(")(input)?;
    Ok((i, Token::Lpar))
}

pub fn rpar<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let (i, _) = tag(")")(input)?;
    Ok((i, Token::Rpar))
}

pub fn sym<'a>(input: &'a str) -> IResult<&'a str, Token> {
    let (i, res) = recognize(tuple((alpha1, alphanumeric0)))(input)?;
    let res = String::from(res);
    Ok((i, Token::Sym(res)))
}

fn register_sink<S: 'static>(mut sink: S) -> Result<(), JsValue>
where
    S: Sink<String> + Unpin,
    <S as Sink<String>>::Error: std::fmt::Debug,
{
    let prompt = Rc::new(
        window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .get_element_by_id("prompt-control")
            .unwrap_throw()
            .dyn_into::<HtmlInputElement>()?,
    );
    let chunks = Rc::new(
        window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .get_element_by_id("chunks-control")
            .unwrap_throw()
            .dyn_into::<HtmlTextAreaElement>()?,
    );
    let closure = {
        let prompt = prompt.clone();
        let chunks = chunks.clone();
        Closure::wrap(Box::new(move |event: KeyboardEvent| {
            if let "Enter" = event.key().as_ref() {
                prompt.set_disabled(true);
                let value = prompt.value();
                chunks.set_value(format!("{}{}", chunks.value(), value).as_ref());
                let future = sink.send(value);
                let result = futures::executor::block_on(future);
                match result {
                    Ok(()) => {
                        prompt.set_value("");
                        prompt.set_disabled(false);
                    },
                    Err(err) => {
                        panic!("{:?}", err);
                    },
                }
            }
        }) as Box<dyn FnMut(_)>)
    };
    prompt.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}

async fn handle_stream<S>(stream: S) -> Result<(), JsValue>
where
    S: Stream<Item = String> + Unpin,
{
    let tokens = window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .get_element_by_id("tokens-control")
        .unwrap_throw()
        .dyn_into::<HtmlSelectElement>()?;
    let options = tokens.options();
    let mut tokens = NomStream::new(stream.map::<Result<_, ()>, _>(Ok), token);
    while let Some(tok) = tokens.next().await {
        let option = HtmlOptionElement::new_with_text(format!("{:?}", tok).as_ref())?;
        options.add_with_html_option_element(&option)?;
    }
    Ok(())
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let (sink, stream) = mpsc::unbounded::<String>();
    register_sink(sink)?;
    handle_stream(stream).await?;
    Ok(())
}
