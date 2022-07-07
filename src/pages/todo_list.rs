use crate::components::atoms::button::Button;
use crate::components::atoms::input::Input;
use crate::components::icons::check::CheckIcon;
use crate::components::icons::delete::DeleteIcon;
use gloo_console as console;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use stylist::style;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let style = style!(
        r#"
          text-align: center;
          flex: 1 1 auto;
          display: flex;
          flex-direction: column;
          align-items: center;
          padding-top: 40px;


          .center {
            width: 400px;
            box-shadow: 0 0 3px 2px #f0f1f2;
            padding: 24px;
            margin-bottom: 40px;
          }

          h1 {
            padding-bottom: 25px;
            font-size: 25px;
            font-weight: bold;
          }

          input {
            border-right-color: transparent;
            border-right-width: 1px;
            border-top-right-radius:0px;
            border-bottom-right-radius:0px;
            margin-right: -1px;
          }

          button {
            border-top-left-radius:0px;
            border-bottom-left-radius:0px;
            z-index: 1;
          }

          ul {
            padding-top: 30px;
          }

          li {
            margin-bottom: 0px;
            height: 40px;
            border-bottom: 1px solid #eee;
            text-align: left;
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 5px 0;
          }

          .check-btn, .delete-btn {
            display: inline-flex;
            width: 30px;
            height: 30px;
            cursor: pointer;
            padding: 4px;
            justify-content: center;
            align-items: center;
            font-size: 20px;
          }

          .delete-btn {
            color: red;
          }

          .completed .check-btn {
            color: green;
          }

          .completed .todo-title {
            text-decoration:line-through;
          }
      "#
    )
    .expect("Failed to mount style!");

    let todos = use_state(|| Todos::new());

    let input_ref = use_node_ref();

    // add button and `Enter` key world
    let add_todo = |todos: &UseStateHandle<Todos>, input_ref: &NodeRef| {
        if let Some(input) = input_ref.cast::<HtmlInputElement>() {
            let value = input.value();

            if value.trim().is_empty() {
                return;
            }

            let mut todos_state = todos.deref().clone();
            todos_state.name = value.to_owned();
            let todo = Todo::new(value.trim().to_owned());
            todos_state.add(todo);
            todos.set(todos_state);

            input.set_value("");
        }
    };

    let handle_add = {
        let input_ref = input_ref.clone();

        let todos = todos.clone();
        Callback::from(move |_| {
            add_todo(&todos, &input_ref);
        })
    };

    let handle_input = Callback::from(|e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let value = input.value();
        console::log!("Received update", value);
        // console::log!("Received update", e);
    });

    let handle_key_down = {
        let input_ref = input_ref.clone();
        let todos = todos.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key().to_owned() == "Enter" {
                add_todo(&todos, &input_ref);
            }
        })
    };

    let handle_toggle_complete = |idx: usize| {
        let todos = todos.clone();
        Callback::from(move |_| {
            let mut todos_state = todos.deref().clone();
            todos_state.toggle_completed(idx);
            todos.set(todos_state);
        })
    };

    let handle_delete = |idx: usize| {
        let todos = todos.clone();
        Callback::from(move |_| {
            console::log!(idx);
            let mut todos_state = todos.deref().clone();
            todos_state.remove(idx);
            todos.set(todos_state);
        })
    };

    html! {
      <div class={style}>
        <div class="center">
          <h1>{"Todo App"}</h1>
          <div class="flex">
            <Input placeholder="What needs to be done?" oninput={handle_input} onkeydown={handle_key_down} ref={input_ref}/>
            <Button onclick={handle_add}>{"Add"}</Button>
          </div>
          <ul>
            {
              todos.todos.iter().enumerate().map(|(idx, entry)| {
                let completed = if entry.completed {
                  Some("completed")
                } else {
                    None
                };

                html! {
                  <li key={&*entry.description} class={classes!(completed)}>
                    <span class="todo-title">{&*entry.description}</span>
                    <div>
                      <span class="check-btn" onclick={handle_toggle_complete(idx)}>
                        <CheckIcon checked={entry.completed}/>
                      </span>
                      <span class="delete-btn" onclick={handle_delete(idx)}>
                          <DeleteIcon />
                      </span>
                    </div>
                  </li>
                }
              }).collect::<Html>()
            }
          </ul>
        </div>
      </div>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    pub description: String,
    pub completed: bool,
    pub editing: bool,
}

impl Todo {
    fn new(description: String) -> Todo {
        Todo {
            description,
            completed: false,
            editing: false,
        }
    }
    // fn toggle_completed(&mut self) {
    //     self.completed = !self.completed;
    // }

    // fn toggle_edit(&mut self) {
    //     self.editing = !self.editing;
    // }

    // fn set_description(&mut self, description: String) {
    //     self.description = description;
    // }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todos {
    todos: Vec<Todo>,
    name: String,
}

impl Todos {
    fn new() -> Self {
        Self {
            todos: vec![],
            name: String::new(),
        }
    }

    fn add(&mut self, todo: Todo) {
        self.todos.insert(0, todo);
    }

    fn remove(&mut self, idx: usize) {
        self.todos.remove(idx);
    }

    fn toggle_completed(&mut self, idx: usize) {
        self.todos[idx].completed = !self.todos[idx].completed;
    }
}
