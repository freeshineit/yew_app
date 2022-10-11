## yew app


### Use

```bash

# install trunk
brew install trunk

# development
# https://trunkrs.dev/
trunk serve

# release
trunk build --release

# http-server
cd dist && http-server -p 8000 .

# docker nginx deploy
# http://localhost:8000
docker-compose up -d

## github page deploy docs
trunk build --release --public-url=/yew_app/
```



### Route



### Todo List

![todo list](./todo_list.png)