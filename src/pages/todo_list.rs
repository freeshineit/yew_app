use crate::components::atoms::button::Button;
use crate::components::atoms::input::Input;
use crate::components::icons::check::CheckIcon;
use crate::components::icons::delete::DeleteIcon;
use crate::utils::storage::LocalStorage;
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
          padding: 40px 20px;

          .center {
            width: 100%;
            max-width: 600px;
            background: #fff;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
            border-radius: 8px;
            padding: 32px;
            margin-bottom: 40px;
          }

          h1 {
            padding-bottom: 32px;
            font-size: 28px;
            font-weight: 600;
            color: #000000d9;
          }

          .input-group {
            display: flex;
            gap: 8px;
            margin-bottom: 8px;
          }

          input {
            flex: 1;
            border-radius: 4px;
          }

          button {
            border-radius: 4px;
            min-width: 80px;
          }

          .stats {
            display: flex;
            justify-content: space-between;
            padding: 16px 0;
            font-size: 13px;
            color: #00000073;
            border-bottom: 1px solid #f0f0f0;
          }

          ul {
            padding-top: 16px;
          }

          li {
            margin-bottom: 8px;
            min-height: 48px;
            border-bottom: 1px solid #f0f0f0;
            text-align: left;
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 12px 8px;
            transition: background-color 0.2s;
          }

          li:hover {
            background-color: #fafafa;
          }

          .todo-title {
            flex: 1;
            padding: 0 12px;
            word-break: break-word;
            line-height: 1.5;
          }

          .check-btn, .delete-btn {
            display: inline-flex;
            width: 32px;
            height: 32px;
            cursor: pointer;
            padding: 6px;
            justify-content: center;
            align-items: center;
            font-size: 20px;
            border-radius: 4px;
            transition: all 0.2s;
          }

          .check-btn:hover {
            background-color: #f0f0f0;
          }

          .delete-btn {
            color: #ff4d4f;
          }

          .delete-btn:hover {
            background-color: #fff1f0;
          }

          .completed .check-btn {
            color: #52c41a;
          }

          .completed .todo-title {
            text-decoration: line-through;
            color: #00000040;
          }

          .empty-state {
            padding: 60px 20px;
            text-align: center;
            color: #00000040;
            font-size: 14px;
          }

          @media (max-width: 768px) {
            padding: 20px 16px;
            
            .center {
              padding: 24px 16px;
            }

            h1 {
              font-size: 24px;
            }
          }
      "#
    ).expect("Failed to create style");

    let todos = use_state(|| {
        // 从 localStorage 加载 todos
        if let Some(stored) = LocalStorage::get("todos") {
            if let Ok(todos) = serde_json::from_str::<Todos>(&stored) {
                return todos;
            }
        }
        Todos::new()
    });

    // 保存到 localStorage
    {
        let todos_json = serde_json::to_string(&*todos).ok();
        use_effect_with(todos_json, move |json| {
            if let Some(json_str) = json {
                let _ = LocalStorage::set("todos", json_str);
            }
            || ()
        });
    }

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
      <div class={style.get_class_name().to_string()}>
        <div class="center">
          <h1>{"Todo App"}</h1>
          <div class="input-group">
            <Input placeholder="What needs to be done?" oninput={handle_input} onkeydown={handle_key_down} input_ref={input_ref}/>
            <Button onclick={handle_add}>{"Add"}</Button>
          </div>
          <div class="stats">
            <span>{format!("Total: {}", todos.todos.len())}</span>
            <span>{format!("Completed: {}", todos.todos.iter().filter(|t| t.completed).count())}</span>
            <span>{format!("Active: {}", todos.todos.iter().filter(|t| !t.completed).count())}</span>
          </div>
          {
            if todos.todos.is_empty() {
              html! {
                <div class="empty-state">
                  {"No todos yet. Add one above! 📝"}
                </div>
              }
            } else {
              html! {
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
                          <span class="check-btn" onclick={handle_toggle_complete(idx)}>
                            <CheckIcon checked={entry.completed}/>
                          </span>
                          <span class="todo-title">{&*entry.description}</span>
                          <span class="delete-btn" onclick={handle_delete(idx)}>
                              <DeleteIcon />
                          </span>
                        </li>
                      }
                    }).collect::<Html>()
                  }
                </ul>
              }
            }
          }
        </div>
      </div>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
