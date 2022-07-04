#!/usr/bin/env python3
# *-* coding:utf-8 *-*

###############################################################################
# Nodes and parser for the Skribi language.                                   #
###############################################################################

# Imports
from src.skribi.tokens import Token
from src.skribi.custom_exception import SkribiException
from src.skribi.skribi_file import ScopeStack


# --------------------------------------------------------------------------- #
# Nodes                                                                       #
# --------------------------------------------------------------------------- #

scope_stack = ScopeStack()


# class Node that can be evaluated
class EvaluableNode:
    def __init__(self, token):
        self.token = token

    def evaluate(self):
        pass

    def copy(self):
        return EvaluableNode(self.token.copy())


class ExecutableNode:
    def __init__(self, token):
        self.token = token

    def execute(self):
        pass

    def copy(self):
        return ExecutableNode(self.token.copy())


# Number node
class NumberNode(EvaluableNode):
    # Constructor with token
    def __init__(self, token: Token):
        super().__init__(token)
        self.token = token

    # String representation
    def __str__(self):
        return str(self.token.value)

    # Evaluate
    def evaluate(self):
        return self.token.value

    def copy(self):
        return NumberNode(self.token.copy())


# Operator node
class OperatorNode(EvaluableNode):
    # Constructor with token and 2 left nodes (reverse polish notation)
    def __init__(self, token: Token, left1, left2):
        super().__init__(token)
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
            return SkribiException("Unknown operator: " + str(self.token.value), "evaluation", scope_stack.get_trace())

    def copy(self):
        return OperatorNode(self.token.copy(), self.left1.copy(), self.left2.copy())


# Node for a variable declaration
class VariableNode(ExecutableNode):
    """
    Node for a variable declaration. Syntax: [name]:<optional type> = [value]
    """

    # constructor with value and name and optional type
    def __init__(self, name: Token, value: EvaluableNode, token, type_: Token = None):
        super().__init__(token)
        self.name = name
        self.value = value
        self.type_ = type_

    # String representation
    def __str__(self):
        if self.type_ is None:
            return str(self.name.value) + " = " + str(self.value.evaluate())
        else:
            return str(self.name.value) + ":" + str(self.type_.value) + " = " + str(self.value.evaluate())

    # Execute TODO : il faut avant faire les variables
    def execute(self):
        if self.type_ is None:
            if scope_stack.get_current_scope().check_name(self.name.value):
                scope_stack.get_current_scope()\
                    .set_variable(self.name.value, self.value.evaluate(), scope_stack.get_current_scope())
            else:
                scope_stack.get_current_scope()\
                    .create_variable(self.name.value, self.value.evaluate(), scope_stack.get_current_scope())
        else:
            pass

    def copy(self):
        return VariableNode(self.name.copy(), self.value.copy(), self.token.copy(), self.type_.copy())


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
        self.current_line = 0

    # Parse
    def parse(self, tokens: list):
        self.tokens = tokens
        self.index = 0
        self.current_token = None
        self.current_node = None
        self.current_line = 0
        self.next_token()
        return self.parse_expr()

    # Parse expression
    def parse_expr(self):
        return self.parse_math_expr()

    # Parse math expression
    def parse_math_expr(self):
        # si le token n'est pas un FLOAT ou un INT, je lève une exception
        if self.current_token.type not in ["FLOAT", "INT"]:
            return SkribiException("Expected a number, got: " + str(self.current_token.value), "parsing")
        current_operator = NumberNode(self.current_token)
        self.next_token()
        # tant que le token est un FLOAT ou un INT ou une opération, je répète l'opération : si le token est un FLOAT
        # ou un INT, je l'ajoute à la pile sinon j'enlève de la pile le dernier élément et je prends un NumberNode
        numbers_pile = []
        while self.current_token.type == "FLOAT" or self.current_token.type == "INT" \
                or self.current_token.type == "OPERATOR":
            if self.current_token.type == "FLOAT" or self.current_token.type == "INT":
                numbers_pile.append(NumberNode(self.current_token))
                self.next_token()
            else:
                if len(numbers_pile) < 1:
                    return SkribiException("Expected a number, got: " + str(self.current_token.value), "parsing")
                current_operator = OperatorNode(self.current_token, numbers_pile.pop().copy(), current_operator.copy())
                self.next_token()
        if len(numbers_pile) > 0:
            return SkribiException("Missing operator", "parsing")
        return current_operator

    def next_token(self):
        if self.index >= len(self.tokens):
            self.current_token = Token(None, None)
            return
        self.current_token = self.tokens[self.index]
        self.index += 1
        # si le token est une nouvelle ligne, ajouter une ligne au compteur de ligne
        if self.current_token.type == "NEWLINE":
            self.current_line += 1
            # je ne passe pas au prochain token pour que le programme puisse donner une erreur

        return
