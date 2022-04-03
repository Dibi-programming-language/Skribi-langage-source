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
