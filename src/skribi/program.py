# ================== #
# class of a program #
# ================== #

from tokens import *


class Program:

    def analyse(self, file):
        """
        Execute the file
        :param file:
        :return:
        """

        # first step: transform the file into a list of tokens
        lexer = Lexer(file)
        for token in lexer:
            print(token)

        # second step: analyse the list of tokens
        # parser = Parser(lexer.tokens)
        # parser.analyse()

        # third step: execute the program
        # interpreter = Interpreter(parser.program)
        # interpreter.analyse()
        pass
