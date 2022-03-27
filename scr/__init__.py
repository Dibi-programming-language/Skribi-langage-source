##################
# Skribi's shell #
##################

# main scope of shell
import os

scope = True
path = 'scr/'

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
        print('\t- exec')
        print('\t- run')

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
