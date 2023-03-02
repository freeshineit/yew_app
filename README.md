## yew app

![build](https://github.com/freeshineit/yew_app/workflows/build/badge.svg)

### Use

```bash

# install trunk
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



### Todo List

![todo list](./todo_list.png)