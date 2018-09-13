## 使用|| &&等逻辑运算符让代码更简洁
```bash
[ -d dir ] || mkdir -p dir
```
而不要使用
```bash
if [ ! -d dir ]; then
    mkdir -p dir
fi
```
