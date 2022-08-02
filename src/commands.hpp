#pragma once

/**
 * @brief Displays help message, describing the available
 * commands
 */
void print_help();

/**
 * @brief Add a todo in the database
 * 
 * @param name The name of the todo
 * @param due_date The date at which the todo must be completed
 * @return int The id of the new todo
 */
int add_todo(std::string name, time_t due_date);

/**
 * @brief Marks a todo as completed
 * 
 * @param id The id of the todo to complete
 * @return true When the todo was successfully completed
 * @return false In case of error.
 */
bool complete_todo(int id);

/**
 * @brief Deletes a todo from the database
 * 
 * @param id The id of the todo to delete
 * @return true When the todo was successfully deleted
 * @return false In case of error.
 */
bool delete_todo(int id);
