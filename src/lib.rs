use js_sys::*;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let storage_key = "paint-plus";
    // let id = "paint-plus-image-el";
    let width = 640;
    let height = 480;
    let initial_line_width = 3;

    let document = window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(width);
    canvas.set_height(height);
    canvas.style().set_property("border", "solid")?;

    let controls_div: HtmlDivElement = document
        .create_element("div")?
        .dyn_into::<HtmlDivElement>()?;
    document.body().unwrap().append_child(&controls_div)?;

    let color_picker_input: HtmlInputElement = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;
    color_picker_input.set_type("color");
    let color_picker_label = document
        .create_element("label")?
        .dyn_into::<HtmlLabelElement>()?;
    color_picker_label.set_inner_text("Pick a color");
    controls_div.append_child(&color_picker_input)?;
    controls_div.append_child(&color_picker_label)?;

    let line_width_input = document
        .create_element("input")?
        .dyn_into::<HtmlInputElement>()?;
    line_width_input.set_type("range");
    line_width_input.set_min("1");
    line_width_input.set_max("10");
    line_width_input.set_value(initial_line_width.to_string().as_str());
    let line_width_label = document
        .create_element("label")?
        .dyn_into::<HtmlLabelElement>()?;
    line_width_label.set_inner_text("Select stroke width");
    controls_div.append_child(&line_width_input)?;
    controls_div.append_child(&line_width_label)?;

    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    context.set_line_cap("round");
    context.set_line_join("round");

    let image_el = document
        .create_element("img")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();

    let local_storage = window().unwrap().local_storage().unwrap().unwrap();
    if let Some(stored_image) = local_storage.get_item(storage_key).unwrap() {
        console::log_1(&"in if".into());
        image_el.set_src(&stored_image);
    }

    {
        let context = context.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
            console::log_1(&"image onload".into());
            let image_el_from_event = event
                .current_target()
                .unwrap()
                .dyn_into::<HtmlImageElement>()
                .unwrap();
            context
                .draw_image_with_html_image_element(&image_el_from_event, 0.0, 0.0)
                .unwrap();
        });
        image_el.add_event_listener_with_callback("load", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: InputEvent| {
            let input = event
                .current_target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            context.set_stroke_style(&wasm_bindgen::JsValue::from_str(input.value().as_str()));
        });
        color_picker_input
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;

        closure.forget();
    }
    {
        let context = context.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: InputEvent| {
            let input = event
                .current_target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            context.set_line_width(input.value().parse::<f64>().unwrap());
        });
        line_width_input
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;

        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let save = {
            let canvas = canvas.clone();
            move || {
                let local_window = web_sys::window().unwrap();
                let local_storage = local_window.local_storage().unwrap().unwrap();
                let image_data_url = canvas.to_data_url();
                if let Err(e) =
                    local_storage.set_item(storage_key, &image_data_url.unwrap().as_str())
                {
                    console::log_1(&e);
                }
            }
        };
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();

            save();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
