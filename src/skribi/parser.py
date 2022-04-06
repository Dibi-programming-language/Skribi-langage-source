###############################################################################
# Nodes and parser for the Skribi language.                                   #
###############################################################################

# Imports
from src.skribi.tokens import Token
from src.skribi.custom_exception import SkribiException, ExceptionLine


# --------------------------------------------------------------------------- #
# Nodes                                                                       #
# --------------------------------------------------------------------------- #

# Number node
class NumberNode(object):
    # Constructor with token
    def __init__(self, token: Token):
        self.token = token

    # String representation
    def __str__(self):
        return str(self.token.value)

    # Evaluate
    def evaluate(self):
        return self.token.value


# Operator node
class OperatorNode(object):
    # Constructor with token and 2 left nodes (reverse polish notation)
    def __init__(self, token: Token, left1, left2):
        self.token = token
        self.left1 = left1
        self.left2 = left2

    # String representation
    def __str__(self):
        return str(self.token.value) + "(" + str(self.left1) + ", " + str(self.left2) + ")"

    # Evaluate
    def evaluate(self):
        if self.token.value == "+":
            return self.left1.evaluate() + self.left2.evaluate()
        elif self.token.value == "-":
            return self.left1.evaluate() - self.left2.evaluate()
        elif self.token.value == "*":
            return self.left1.evaluate() * self.left2.evaluate()
        elif self.token.value == "/":
            return self.left1.evaluate() / self.left2.evaluate()
        elif self.token.value == "^":
            return self.left1.evaluate() ** self.left2.evaluate()
        else:
            return SkribiException("Unknown operator: " + str(self.token.value), "evaluation")


# --------------------------------------------------------------------------- #
# Parser                                                                      #
# --------------------------------------------------------------------------- #

# Parser class
class Parser:

    def __init__(self):
        self.tokens = []
        self.index = 0
        self.current_token = None
        self.current_node = None
        self.current_line = None
        self.current_column = None

    # Parse
    def parse(self, tokens: list):
        self.tokens = tokens
        self.index = 0
        self.current_token = None
        self.current_node = None
        self.current_line = None
        self.current_column = None
        self.next_token()
        return self.parse_expr()

    # Parse expression
    def parse_expr(self):
        self.current_node = self.parse_term()
        while self.current_token.value in ["+", "-", "*", "/", "^"]:
            self.current_node = OperatorNode(self.current_token, self.current_node, self.parse_term())
            self.next_token()
        return self.current_node

    # Parse term
    def parse_term(self):
        self.current_node = self.parse_factor()
        while self.current_token.value in ["+", "-", "*", "/", "^"]:
            self.current_node = OperatorNode(self.current_token, self.current_node, self.parse_factor())
            self.next_token()
        return self.current_node

    # Parse factor
    def parse_factor(self):
        if self.current_token.type == "number":
            self.current_node = NumberNode(self.current_token)
            self.next_token()
        elif self.current_token.type == "left_parenthesis":
            self.next_token()
            self.current_node = self.parse_expr()
            if self.current_token.type == "right_parenthesis":
                self.next_token()
            else:
                raise SkribiException("Missing right parenthesis", "parsing")
        else:
            raise SkribiException("Unexpected token: " + str(self.current_token.value), "parsing")
        return self.current_node

    def next_token(self):
        if self.index >= len(self.tokens):
            self.current_token = Token(None, None)
            return
        self.current_token = self.tokens[self.index]
        self.index += 1
        self.current_line = self.current_token.line
        self.current_column = self.current_token.column
        return
