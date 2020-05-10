# sk 补充知识分支
### 补充知识
- Option
- struct size
- cli
- queue
- file
- stack
- link stack

#### Option
- base
```rust
pub enum Option<T> {
    Some(T),
    None,
}
```
- 用法
    - 初始化值
    - 作为整个输入内没有定义的函数的放回值
    - 作为放回值,用None表示出现的简单错误
    - 作为结构体的可选字段
    - 作为结构体中可借出或则是可载入的字段
    - 作为函数的可选参数
    - 代表空指针
    - 用做复杂情况的返回值
- api
    - `take()` 移出当前数据 并设置为None
    - `is_some()` 有值返回true
    - `is_none()`


### 获取命令行参数
``` 
use std::env

let args: Vec<String> = new::args().collect(); // 类似golang的 os.args()
```