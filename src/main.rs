use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::prelude::*;
use yew::{use_state, Callback, Properties};

#[derive(Properties, Clone, Default, PartialEq)]
struct TaskProps {
    title: String,
    body: String,
}

fn event_cast<T: JsCast>(e: InputEvent) -> T {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    event_target.dyn_into().unwrap_throw()
}

#[function_component(Task)]
fn task(props: &TaskProps) -> Html {
    let clicked = use_state(|| false);
    let onclick = {
        let clicked = clicked.clone();
        Callback::from(move |_| clicked.set(!*clicked))
    };
    let body = use_state(|| props.body.clone());
    let oninput = {
        let body = body.clone();
        Callback::from(move |event: InputEvent| {
            body.set(event_cast::<HtmlTextAreaElement>(event).value())
        })
    };
    html! {
        <div class="card">
            <div class="card-header" {onclick}>
            <p class="card-header-title">{&props.title}</p>
            </div>
        {
            if *clicked {
                html!{
                    <div class="card-content">
                        <textarea placeholder="Todo body" class="textarea" value={(*body).clone()} {oninput}>
                        </textarea>
                    </div>
                }
            } else {
                html!{<></>}
            }
        }
        </div>
    }
}

#[function_component(List)]
fn list() -> Html {
    let tasks = use_state(|| Vec::<TaskProps>::new());
    let input_value = use_state(|| "".to_owned());
    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |ievent: InputEvent| {
            input_value.set(event_cast::<HtmlInputElement>(ievent).value())
        })
    };
    let onclick = {
        let tasks = tasks.clone();
        let input_value = input_value.clone();
        Callback::from(move |_: MouseEvent| {
            let mut new_tasks = (*tasks).clone();
            new_tasks.push(TaskProps {
                title: (*input_value).clone(),
                body: "".to_owned(),
            });
            tasks.set(new_tasks);
        })
    };
    html! {
        <div class="box" style="max-width:450px;">
        {
            (*tasks).iter().map(|ref task| html!{
                <Task title={task.title.clone()} body={task.body.clone()} />
            }).collect::<Html>()
        }
        <div class="columns">
            <input class="input column is-four-fifths" placeholder="Add new task" {oninput}/>
            <button class="button column" {onclick}>{"+"}</button>
        </div>
        </div>
    }
}

fn main() {
    yew::start_app::<List>();
}
