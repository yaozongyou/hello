## 使用逻辑运算符让代码更简洁
```bash
[[ -d /path/to/some/dir ]] || mkdir -p /path/to/some/dir
```
而不要使用
```bash
if [[ ! -d /path/to/some/dir ]]; then
  mkdir -p /path/to/some/dir
fi
```
