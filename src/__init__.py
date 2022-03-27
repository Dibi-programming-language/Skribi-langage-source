##################
# Skribi's shell #
##################

import os
import skribi


# main scope of shell

scope = True
path = 'src/'

while scope:

    # welcome message
    print("Skribi's Shell")
    print("Type 'help' for help")

    # user input
    user_input = input('(' + path + ') -> ')

    # exit
    if user_input == 'exit':
        print('Bye!')
        scope = False
        break

    # help
    elif user_input == 'help':
        print('\nCommands:')
        print('\t- exit: exit the shell')
        print('\t- help: show this help')
        print('\t- ls: list files in current directory')
        print('\t- cd: change directory')
        print('\t- exec: execute a file in Skribi')
        print('\t- run: run a line of code in Skribi')

    # list files
    elif user_input == 'ls':
        print('\nFiles:')
        for file in os.listdir(path):
            print(file)

    # change directory
    elif user_input == 'cd':
        new_path = input('\nNew path: ')
        if os.path.isdir(new_path):
            path = new_path
            print('\nChanged directory to ' + path)
        else:
            print('\nDirectory not found!')

    # execute file
    elif user_input == 'exec':
        file_name = input('\nFile name: ')
        if os.path.isfile(path + file_name):
            # read file
            with open(path + file_name, 'r') as f:
                file_content = f.read()
            # execute file
            skribi.execute(file_content, True)
        else:
            print('\nFile not found!')

    # run line of code
    elif user_input == 'run':
        line = input("\nLine: ")
        skribi.execute(line, False)