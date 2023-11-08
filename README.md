## 半自动设置QQ音乐Cookie
运行程序模拟登录QQ音乐, 获取到Cookie, 设置到QQ音乐API项目里

QQ音乐API项目地址: https://github.com/jsososo/QQMusicApi

### 必须
- chromedriver
- chrome
- QQ音乐API项目地址

### Windows下编译打包
```shell
cargo build --release
```

### 运行
qcookie.exe

### 配置
在可执行文件同级目录下创建`.env`文件
```
driver_path=D:/dev/env/chromedriver-win64/chromedriver.exe
qq_music_api=http://localhost:3300
u=qq
p=password
```

### 注
- 需要在常用的机器上执行, 不然会出现选图验证码