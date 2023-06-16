# TODO
Task Organisation for Doing OK

## Project Description
This project is a cli project that allows users to create a task list and add tasks to it. The user can also view the tasks in the list and mark them as complete. The user can also delete tasks from the list.

Some tasks may have deadlines and the user can set the deadline for the task. The user can also view the tasks that are overdue.

## ⌨️ Commands
The following commands are available to the user:
```
todo -a <task> [-D '<task_description>' -e '<due_date>'] # Adds a task to the list
todo -l [--overdue] # Views the tasks in the list
todo -c <task_id> # Marks a task as done
todo -d <task_id> # Deletes a task from the list
```

## 🧪 Testing
To run tests, simply run the `test.sh` script from the root folder.
It will create a testing database (to avoid unintended modifications in your main one) and delete it once the tests are done.

It will also make sure the tests are not run in parallel as sqlite does not support concurrency.
