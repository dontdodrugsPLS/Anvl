## Anvl - 0.0.1

Anvl is a command-line tool designed to reduce **setup friction** and **mental load** when working on **C** projects, especially in strict or/and educational environments like **Epitech** or **42** school.

It focuses on three things:

- Fast project initialization
- Safe, explicit module reuse
- Kepping your build system honest.

> No magic. No code generators. Only automation and structure.

## Why did I even create Anvl ?

C projects tend to repeat exactly the same problems:

- With 2 to 4 weeks to deliver a project, often while juggling several projects at the same time, you can’t afford to lose hours setting up the same structure again and again.
- Writing your own library is usually a good idea, but reusing it blindly isn’t. Most projects only need a few functions, and you still have to ensure that every reused feature respects the authorized functions defined by the subject.
- Editing a Makefile isn’t hard, but it’s mental overhead. The cleaner solution is not having to even think about it at all.

## What Anvl is not

- Not a build system
- Not a dependency resolver at compile time
- Not a framework
- Not a package manager that hides code

It works with your existing workflow, not against it.

## Typical project creation workflow

`anvl init bin my_project --push`
  -> Fetch "bin" template from your Anvl repo and rename binary, include etc to "my_project"

`anvl list`
  -> List every available modules from your Anvl repo local cache.

`anvl install io str vec --push`
  -> Install "io", "str" and "vec" with all their dependencies from your Anvl repo to the current project

`anvl create c:cli modules/cli/cli --push`
  -> Create "cli.c" inside src/modules/cli using the "C" template named "cli" inside your Anvl repo.
     Create "cli.c" inside tests/modules/cli using the global test template or if defined the personalizated one.
     Automatically add both file inside compilation Makefile, ready to use.

> [!INFO]
>
> As you can see, some commands have the --push option, it simply mean that Anvl will automatically push concerned changes on your repo. 
>
> This is only the visible part of the iceberg to learn more about it, refere too : ...
