#pragma once

enum {
    HELP_ID,
    VERSION_ID,
    ADD_ID,
    LIST_ID,
    REMOVE_ID,
    EDIT_ID,
    SEARCH_ID,
    CLEAR_ID,
};

/**
 * @brief Abstract class for commands
 * This class is the base for all the commands that can be executed
 * by the program
 */
class Command {
    public:
        Command(int id);
        virtual ~Command();
        virtual int execute() = 0;

    private:
        int command_id;
};

/**
 * @brief Displays help message, describing the available
 * commands
 * 
 * @return int 0 if successful, 1 otherwise
 */
class HelpCommand : public Command {
    public:
        HelpCommand();
        ~HelpCommand();
        int execute();
};

/**
 * @brief Displays the version of the program
 * 
 * @return int 0 if successful, 1 otherwise
 */
class VersionCommand : public Command {
    public:
        VersionCommand();
        ~VersionCommand();
        int execute();
};

/**
 * @brief Add a todo in the database
 * 
 * @param name The name of the todo
 * @param due_date The date at which the todo must be completed
 * 
 * @return int The id of the new todo
 */
class addCommand : public Command {
    public:
        addCommand(std::string name, std::string due_date);
        ~addCommand();
        int execute();
    
    private:
        std::string name;
        std::string due_date;
};

/**
 * @brief Marks a todo as completed
 * 
 * @param id The id of the todo to complete
 * @return true When the todo was successfully completed
 * 
 * @return 0 if successful, 1 otherwise
 */
class completeCommand : public Command {
    public:
        completeCommand(int id);
        ~completeCommand();
        int execute();
    
    private:
        int id;
};

/**
 * @brief Deletes a todo from the database
 * 
 * @param id The id of the todo to delete
 * @return true When the todo was successfully deleted
 * 
 * @return 0 if successful, 1 otherwise
 */
class deleteCommand : public Command {
    public:
        deleteCommand(int id);
        ~deleteCommand();
        int execute();
    
    private:
        int id;
};
