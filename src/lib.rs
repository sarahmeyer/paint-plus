use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let initial_line_width = 3;
    
    let controls_div = document.create_element("div")?.dyn_into::<web_sys::HtmlDivElement>()?;
    document.body().unwrap().append_child(&controls_div)?;

    let color_picker_input = document.create_element("input")?.dyn_into::<web_sys::HtmlInputElement>()?;
    color_picker_input.set_type("color");
    let color_picker_label = document.create_element("label")?.dyn_into::<web_sys::HtmlLabelElement>()?;
    color_picker_label.set_inner_text("Pick a color");
    controls_div.append_child(&color_picker_input)?;
    controls_div.append_child(&color_picker_label)?;
    
    let line_width_input = document.create_element("input")?.dyn_into::<web_sys::HtmlInputElement>()?;
    line_width_input.set_type("range");
    line_width_input.set_min("1");
    line_width_input.set_max("10");
    line_width_input.set_value(initial_line_width.to_string().as_str());
    let line_width_label = document.create_element("label")?.dyn_into::<web_sys::HtmlLabelElement>()?;
    line_width_label.set_inner_text("Select stroke width");
    controls_div.append_child(&line_width_input)?;
    controls_div.append_child(&line_width_label)?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::InputEvent| {
            let input = event
                .current_target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(event.data()));
            context.set_stroke_style(&wasm_bindgen::JsValue::from_str(&input.value()));
        });
        color_picker_input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;

        closure.forget();
    }
    {
        let context = context.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::InputEvent| {
            let input = event
                .current_target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            context.set_line_width(input.value().parse::<f64>().unwrap());
        });
        line_width_input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;

        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
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
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
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
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}