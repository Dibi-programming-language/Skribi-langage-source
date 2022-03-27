# ================== #
# class of a program #
# ================== #

class Program:

    def execute(self, file):
        """
        Execute the file
        :param file:
        :return:
        """
        pass


# ====== #
# tokens #
# ====== #

TT_INT = 'TT_INT'
TT_FLOAT = 'FLOAT'
TT_PLUS = 'PLUS'
TT_PLUS_DIBI = 'PLUS_DIBI'

class Token:

    def __init__(self, type_, value):
        self.type = type_
        self.value = value

    def __repr__(self):
        if self.value: return f'{self.type}:{self.value}'
        return f'{self.type}'

# ============== #
# class of LEXER #
# ============== #
