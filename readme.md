<h1 align="center">
  Hornet
</h1>

<h4 align="center">
  A command-line interface for Things 3 power users
</h4>

## Description

Hornet is a powerful command-line interface for Things 3 that enables you to manage your tasks, projects, and areas directly from your terminal. With its simple syntax and minimal design, Hornet makes it easy to view, add, and complete tasks without leaving your workflow.

## Highlights

- View tasks by projects, areas, or tags
- Access built-in lists (Inbox, Today, Anytime, Someday, Logbook)
- Add new tasks with minimal effort
- Complete tasks from the command line
- List all projects, areas, and tags
- Organized and color-coded terminal output
- Fast and lightweight

## Contents

- [Description](#description)
- [Highlights](#highlights)
- [Install](#install)
- [Usage](#usage)
- [Examples](#examples)
- [Features](#features)
- [Development](#development)
- [License](#license)

## Install

Work in progress

## Usage

```
$ hornet --help

  Usage
    $ hornet [<options> ...]

    Commands
      add              Add a new todo to Things3
      projects         List all projects
      areas            List all areas
      tags             List all tags
      done             Mark a todo as complete
      inbox            Show inbox todos
      anytime          Show anytime todos
      someday          Show someday todos
      logbook          Show logbook todos

    Options
      --tag            Show todos with specific tag (use #tagname)
      --project        Show todos for specific project (use /projectname)
      --area           Show todos for specific area (use @areaname)

    Examples
      $ hornet
      $ hornet add "Buy groceries"
      $ hornet add "Review PR #42" --project "Website"
      $ hornet projects
      $ hornet areas
      $ hornet tags
      $ hornet done 1
      $ hornet inbox
      $ hornet #work
      $ hornet /Website
      $ hornet @Personal
```

## Examples

### View Today's Tasks

Running `hornet` without any arguments displays your tasks for today.

### Add a Task

```bash
hornet add "Finish documentation"
```

### Add a Task to a Project

```bash
hornet add "Write unit tests" --project "API Project"
```

### Complete a Task

```bash
hornet done 1
```

### View Tasks by Tag

```bash
hornet #urgent
```

### View Tasks by Project

```bash
hornet /Website
```

### View Tasks by Area

```bash
hornet @Work
```

## Development

To contribute to Hornet:

1. Fork the repository and clone it to your machine
2. Navigate to your local fork: `cd hornet`
3. Install the project dependencies: `cargo build`
4. Make your changes and test them: `cargo run`
5. Submit a pull request with your changes

## License

MIT
