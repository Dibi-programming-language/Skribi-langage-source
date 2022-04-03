# ================== #
# class of a program #
# ================== #
from src.skribi.skribi_file import SkribiFile
from src.skribi.tokens import Lexer


class Program:

    def analyse(self, file: SkribiFile) -> None:
        """
        Execute the file
        :param file:
        :return:
        """

        # first step: transform the file into a list of tokens
        lexer = Lexer(file.get_content(), file)
        for token in lexer:
            print(token)

        # second step: analyse the list of tokens
        # parser = Parser(tokens)
        # parser. ......

        # third step: execute the program
        # interpreter = Interpreter(......)
        # interpreter. ....
