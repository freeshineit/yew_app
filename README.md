## yew app

![build](https://github.com/freeshineit/yew_app/workflows/build/badge.svg)

### Use

```bash

# install trunk
# trunk 0.20.3
brew install trunk

# development
# https://trunkrs.dev/
trunk serve --public-url /

# release
trunk build --release

# http-server
cd dist && http-server -p 8000 .

# docker nginx deploy
# http://localhost:8000
docker-compose up -d

## github page deploy docs
trunk build --release
```



### Route

```rs
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/todo_list")]
    TodoList,
    #[at("/videos")]
    Videos,
    #[not_found]
    #[at("/404")]
    NotFound,
}
```
[Route](./src/lib.rs)


### Todo List

![todo list](./todo_list.png)