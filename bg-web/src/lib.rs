use js_sys::Function;
use web_sys::{window, Document, HtmlElement};

use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(text: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Button {
    document: Document,
    body: HtmlElement,
    message: String,
}

impl Button {
    pub fn new(message: &str) -> Option<Button> {
        let document = window()?.document()?;
        let body = document.body()?;
        Some(Button {
            document,
            body,
            message: String::from(message),
        })
    }
    pub fn append_listener(&self) -> Result<(), JsValue> {
        let button = self.document.create_element("button")?;

        self.body.append_child(&button)?;
        button.set_text_content(Some("click me!!"));

        button.add_event_listener_with_callback("click", {
            let message = format!("{}", self.message);

            &Closure::<dyn Fn()>::new(move || {
                console_log!("from closure: {}", message);
            })
            .into_js_value()
            .dyn_into()?
        })?;

        button.add_event_listener_with_callback(
            "click",
            &Function::new_no_args(
                format!("console.log('from Function: {}')", self.message).as_str(),
            ),
        )?;

        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log!("starting wasm");
    Button::new("hello").map_or(Err(JsValue::from_str("failed")), |doc| {
        doc.append_listener()?;
        Ok(())
    })
}
