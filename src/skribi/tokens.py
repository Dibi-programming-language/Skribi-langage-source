from src.skribi.custom_exception import SkribiException, ExceptionLine
from src.skribi.skribi_file import SkribiFile

# ====== #
# tokens #
# ====== #

TT_INT = 'INT'
TT_FLOAT = 'FLOAT'
TT_STRING = 'STRING'
TT_OPERATOR = 'OPERATOR'
TT_COMPARAISON = 'COMPARAISON'
TT_BINARY_OPERATOR = 'BINARY_OPERATOR'
TT_BRACKET = 'BRACKET'
TT_NEWLINE = 'NEWLINE'
TT_COMMENT = 'COMMENT'
TT_IDENTIFIER = 'IDENTIFIER'
TT_EQUAL = 'EQUAL'


class Token:

    def __init__(self, type_, value):
        self.type = type_
        self.value = value

    def __repr__(self):
        if self.value:
            return f'{self.type}:{self.value}'
        return f'{self.type}'


# ===== #
# lexer #
# ===== #

class Lexer:
    def __init__(self, text, file: SkribiFile):
        self.text = text
        self.pos = 0
        self.current_char = self.text[self.pos]
        self.line = 1
        self.file = file

    def advance(self):
        self.pos += 1
        if self.pos > len(self.text) - 1:
            self.current_char = None
        else:
            self.current_char = self.text[self.pos]

    def skip_whitespace(self):
        while self.current_char is not None and self.current_char.isspace():
            self.advance()

    def integer(self, negative=1):
        result = ''
        while self.current_char is not None and self.current_char.isdigit():
            result += self.current_char
            self.advance()
        if self.current_char == '.':
            result += self.current_char
            self.advance()
            while self.current_char is not None and self.current_char.isdigit():
                result += self.current_char
                self.advance()
            return Token(TT_FLOAT, float(result) * negative)
        return Token(TT_INT, int(result) * negative)

    def string(self, sep='"'):
        result = ''
        backslash = False
        while self.current_char is not None and (self.current_char != sep or backslash):
            result += self.current_char
            if self.current_char == '\\' and not backslash:
                backslash = True
            else:
                backslash = False
            self.advance()
        self.advance()
        return Token(TT_STRING, result)

    def identifier(self):
        result = ''
        while self.current_char is not None and self.current_char.isalnum():
            result += self.current_char
            self.advance()
        return Token(TT_IDENTIFIER, result)

    def get_next_token(self):
        while self.current_char is not None:

            if self.current_char.isspace():
                self.skip_whitespace()
                continue

            if self.current_char.isdigit():
                return self.integer()

            if self.current_char == '+':
                self.advance()
                return Token(TT_OPERATOR, '+')

            if self.current_char == '-':
                self.advance()
                # si le prochain caractère est un chiffre, c'est un nombre négatif
                if self.current_char is not None and self.current_char.isdigit():
                    return self.integer(-1)
                return Token(TT_OPERATOR, '-')

            if self.current_char == '*':
                self.advance()
                return Token(TT_OPERATOR, '*')

            if self.current_char == '/':
                self.advance()
                return Token(TT_OPERATOR, '/')

            if self.current_char == '(':
                self.advance()
                return Token(TT_BRACKET, '(')

            if self.current_char == ')':
                self.advance()
                return Token(TT_BRACKET, ')')

            if self.current_char == '"':
                return self.string()

            if self.current_char == '\n':
                self.line += 1
                self.advance()
                return Token("NEWLINE", self.line)

            if self.current_char == '=':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    return Token(TT_COMPARAISON, '==')
                return Token(TT_EQUAL, '=')

            if self.current_char == '!':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    return Token(TT_COMPARAISON, '!=')
                return Token(TT_BINARY_OPERATOR, '!')

            if self.current_char == '<':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    return Token(TT_COMPARAISON, '<=')
                return Token(TT_COMPARAISON, '<')

            if self.current_char == '>':
                self.advance()
                if self.current_char == '=':
                    self.advance()
                    return Token(TT_COMPARAISON, '>=')
                return Token(TT_COMPARAISON, '>')

            if self.current_char.isalpha():
                return self.identifier()

            return self.error()

        return Token(None, None)

    def error(self):
        return SkribiException(
            f'Invalid character: {self.current_char}', "Tokenizer", [ExceptionLine(self.line, self.file.get_path())])

    def __iter__(self):
        while self.current_char is not None:
            token = self.get_next_token()
            if isinstance(token, SkribiException):
                token.print_complete_error()
                return token
            yield token
