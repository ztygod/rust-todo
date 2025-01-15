### `rust-todo` 项目说明

`rust-todo` 是一个用 Rust 编写的简单命令行待办事项（TODO）管理工具。它允许你通过命令行来添加、查看、删除待办事项，并且可以根据不同的需求个性化输出（如按颜色区分、完成/未完成状态等）。该工具支持多种命令行选项，以帮助你高效管理待办事项。

### 命令行输入规范

```bash
rust-todo [OPTIONS]
rust-todo [OPTIONS] [ARGUMENTS]
```

### 命令行选项

| 选项                           | 描述                                       |
|--------------------------------|--------------------------------------------|
| `--help`, `-h`                 | 提供所有帮助消息                           |
| `--get`, `-g` [index]          | 通过索引 `index` 找到某个 TODO 项         |
| `--list`, `-l` [ARGUMENTS]     | 打印所有 TODO 列表                        |
|                                | `--finished`, `-f`                        | 打印所有已完成的 TODO 列表                |
|                                | `--unfinished`, `-u`                      | 打印所有未完成的 TODO 列表               |
| `--new`, `-n` [NAME]           | 输入新任务名称，创建一个新的 TODO 项      |
| `--clear`, `-c`                | 清空所有 TODO 项                          |
| `--remove`, `-r` [index]       | 通过索引 `index` 删除指定的 TODO 项       |
| `--color`, `-cl` [index] [COLOR] | 通过索引 `index` 设置 TODO 项的个性化颜色 |
| `--done`, `-d` [index]         | 将索引为 `index` 的 TODO 项标记为已完成   |
| `-version`, `-v`               | 输出当前版本号                             |

### 可选颜色

| 颜色      | 符号  |
|-----------|-------|
| `red`     | ⦿    |
| `blue`    | ⦿    |
| `yellow`  | ⦿    |
| `green`   | ⦿    |
| `cyan`    | ⦿    |
| `purple`  | ⦿    |
| `magenta` | ⦿    |

### 示例

#### 1. 创建一个新的 TODO 项

```bash
rust-todo --new "Finish Homework"
```

这会创建一个新的 TODO 项，名称为 `"Finish Homework"`。

#### 2. 查看所有 TODO 列表

```bash
rust-todo --list
```

打印所有的 TODO 项。

#### 3. 查看已完成的 TODO 项

```bash
rust-todo --list --finished
```

打印所有已完成的 TODO 项。

#### 4. 查看未完成的 TODO 项

```bash
rust-todo --list --unfinished
```

打印所有未完成的 TODO 项。

#### 5. 标记 TODO 项为已完成

```bash
rust-todo --done 2
```

将索引为 `2` 的 TODO 项标记为已完成。

#### 6. 设置 TODO 项的颜色

```bash
rust-todo --color 1 red
```

将索引为 `1` 的 TODO 项的颜色设置为红色。

#### 7. 删除 TODO 项

```bash
rust-todo --remove 3
```

删除索引为 `3` 的 TODO 项。

#### 8. 清空所有 TODO 列表

```bash
rust-todo --clear
```

清空所有的 TODO 项。

#### 9. 查看版本号

```bash
rust-todo --version
```

输出当前版本号。

### 项目结构

- `src/`：包含源代码
  - `main.rs`：程序的入口点和命令行解析
  - `todo.rs`：TODO 项的数据结构和方法
- `Cargo.toml`：Cargo 配置文件，用于定义项目依赖和元数据

### 安装和运行

1. **安装 Rust：** 如果尚未安装 Rust，请访问 [Rust 官网](https://www.rust-lang.org/) 安装 Rust。
2. **克隆项目：**

   ```bash
   git clone https://github.com/yourusername/rust-todo.git
   cd rust-todo
   ```

3. **构建项目：**

   ```bash
   cargo build --release
   ```

4. **运行程序：**

   ```bash
   target/release/rust-todo --help
   ```

### 贡献

我们欢迎贡献！如果你有好的建议或 bug 修复，请创建一个 Pull Request。
