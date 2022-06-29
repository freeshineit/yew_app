use crate::components::atoms::button::Button;
use crate::components::atoms::input::Input;
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

    let handle_add = {
        let input_ref = input_ref.clone();

        let todos = todos.clone();
        Callback::from(move |e: MouseEvent| {
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
                        <svg viewBox="0 0 1024 1024" version="1.1"  width="1em" height="1em" fill="currentColor" focusable="false">
                        {
                          if entry.completed {
                            html! {
                              <path d="M433.1 657.7c12.7 17.7 39 17.7 51.7 0l210.6-292c3.8-5.3 0-12.7-6.5-12.7H642c-10.2 0-19.9 4.9-25.9 13.3L459 584.3l-71.2-98.8c-6-8.3-15.6-13.3-25.9-13.3H315c-6.5 0-10.3 7.4-6.5 12.7l124.6 172.8z" />
                            }
                          } else {
                            html!{}
                          }
                        }
                          <path d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32z m-40 728H184V184h656v656z" />
                        </svg>
                      </span>
                      <span class="delete-btn" onclick={handle_delete(idx)}>
                        <svg viewBox="0 0 1024 1024"  width="1em" height="1em" fill="currentColor" focusable="false">
                          <path d="M768 384c-19.2 0-32 12.8-32 32l0 377.6c0 25.6-19.2 38.4-38.4 38.4L326.4 832c-25.6 0-38.4-19.2-38.4-38.4L288 416C288 396.8 275.2 384 256 384S224 396.8 224 416l0 377.6c0 57.6 44.8 102.4 102.4 102.4l364.8 0c57.6 0 102.4-44.8 102.4-102.4L793.6 416C800 396.8 787.2 384 768 384z" />
                          <path d="M460.8 736l0-320C460.8 396.8 448 384 435.2 384S396.8 396.8 396.8 416l0 320c0 19.2 12.8 32 32 32S460.8 755.2 460.8 736z" />
                          <path d="M627.2 736l0-320C627.2 396.8 608 384 588.8 384S563.2 396.8 563.2 416l0 320C563.2 755.2 576 768 588.8 768S627.2 755.2 627.2 736z"/>
                          <path d="M832 256l-160 0L672 211.2C672 166.4 633.6 128 588.8 128L435.2 128C390.4 128 352 166.4 352 211.2L352 256 192 256C172.8 256 160 268.8 160 288S172.8 320 192 320l640 0c19.2 0 32-12.8 32-32S851.2 256 832 256zM416 211.2C416 198.4 422.4 192 435.2 192l153.6 0c12.8 0 19.2 6.4 19.2 19.2L608 256l-192 0L416 211.2z" />
                          </svg>
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
    ///
    fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    fn toggle_edit(&mut self) {
        self.editing = !self.editing;
    }

    fn set_description(&mut self, description: String) {
        self.description = description;
    }
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

// type List = Vec<Item>;

// impl List {
//     fn new(items: Vec<Item>) -> List {
//         items
//     }

//     fn remove(&mut self, index: usize) {
//         self.remove(index);
//     }

//     fn add(&mut self, item: Item) {
//         self.push(item);
//     }
// }

// /// https://github.com/yewstack/yew/blob/yew-v0.19.3/examples/todomvc/src/state.rs
// #[derive(Debug, Serialize, Deserialize)]
// struct Todo {
//     pub entries: Vec<Entry>,
//     pub filter: Filter,
//     pub edit_value: String,
// }

// impl Todo {
//     fn total(&self) -> usize {
//         self.entries.len()
//     }

//     fn total_completed(&self) -> usize {
//         self.entries
//             .iter()
//             .filter(|e| Filter::Completed.fits(e))
//             .count()
//     }

//     fn is_all_completed(&self) -> bool {
//         let mut filtered_iter = self
//             .entries
//             .iter()
//             .filter(|e| Filter::Completed.fits(e))
//             .peekable();

//         if filtered_iter.peek().is_none() {
//             return false;
//         }

//         filtered_iter.all(|e| e.completed)
//     }

//     fn clear_completed(&mut self) {
//         let entries = self
//             .entries
//             .drain(..)
//             .filter(|e| Filter::Active.fits(e))
//             .collect();
//         self.entries = entries
//     }

//     fn toggle(&mut self, idx: usize) {
//         let filter = self.filter;
//         let entry = self
//             .entries
//             .iter_mut()
//             .filter(|e| filter.fits(e))
//             .nth(idx)
//             .unwrap();

//         entry.completed = !entry.completed;
//     }

//     fn toggle_all(&mut self, value: bool) {
//         for entry in &mut self.entries {
//             if self.filter.fits(&entry) {
//                 entry.completed = value
//             }
//         }
//     }

//     fn toggle_edit(&mut self, idx: usize) {
//         let filter = self.filter;

//         let entry = self
//             .entries
//             .iter_mut()
//             .filter(|e| filter.fits(e))
//             .nth(idx)
//             .unwrap();

//         entry.editing = !entry.editing;
//     }

//     fn clear_all_edit(&mut self) {
//         for entry in &mut self.entries {
//             entry.editing = false;
//         }
//     }

//     fn completed_edit(&mut self, idx: usize, val: String) {
//         if val.is_empty() {
//             self.remove(idx);
//         } else {
//             let filter = self.filter;
//             let entry = self
//                 .entries
//                 .iter_mut()
//                 .filter(|e| filter.fits(e))
//                 .nth(idx)
//                 .unwrap();

//             entry.description = val;
//             entry.editing = !entry.editing;
//         }
//     }

//     fn remove(&mut self, idx: usize) {
//         let idx = {
//             let entries = self
//                 .entries
//                 .iter()
//                 .().map()
//                 .filter(|&(_, e)| self.filter.fits(e))
//                 .collect::<Vec<_>>();

//             let &(idx, _) = entries.get(idx).unwrap();
//             idx
//         };

//         self.entries.remove(idx);
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Entry {
//     description: String,
//     completed: bool,
//     editing: bool,
// }

// #[derive(Debug, Serialize, Deserialize, Copy, Clone)]
// enum Filter {
//     All,
//     Active,
//     Completed,
// }

// impl Filter {
//     fn fits(&self, entry: &Entry) -> bool {
//         match &self {
//             Filter::All => true,
//             Filter::Active => !entry.completed,
//             Filter::Completed => entry.completed,
//         }
//     }

//     fn as_href(&self) -> &'static str {
//         match &self {
//             Filter::All => "#/",
//             Filter::Active => "#/active",
//             Filter::Completed => "#/completed",
//         }
//     }
// }
