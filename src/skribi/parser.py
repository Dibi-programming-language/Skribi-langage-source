#!/usr/bin/env python3
# *-* coding:utf-8 *-*

###############################################################################
# Nodes and parser for the Skribi language.                                   #
###############################################################################

# Imports
from skribi.tokens import Token
from skribi.custom_exception import SkribiException
from skribi.skribi_file import ScopeStack


# --------------------------------------------------------------------------- #
# Nodes                                                                       #
# --------------------------------------------------------------------------- #

scope_stack = ScopeStack()


# class Node that can be evaluated
class EvaluableNode:
    def __init__(self, token):
        self.token = token
        
    # String Representation
    def __str__(self):
        return str(self.__dict__)
    
    def __repr__(self):
        return str(self.__dict__)

    def evaluate(self):
        pass

    def copy(self):
        return EvaluableNode(self.token.copy())


class ExecutableNode:
    def __init__(self, token):
        self.token = token
        
    # String Representation
    def __str__(self):
        return str(self.__dict__)
    
    def __repr__(self):
        return str(self.__dict__)

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
    
    def __repr__(self):
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
    
    def __repr__(self):
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
        
    def __repr__(self):
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
        
    # String Representation
    def __str__(self):
        return str(self.__dict__)
    
    def __repr__(self):
        return str(self.__dict__)

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

        # création de l'array qui va stocker les tokens du calcul
        calc = []

        # remplissage de l'array calc
        i = 0
        while self.current_token.type in ["FLOAT", "INT", "OPERATOR"]:
            if self.current_token.type == "OPERATOR":
                calc.append(OperatorNode(self.current_token,None,None))
            else:
                calc.append(NumberNode(self.current_token))
            self.next_token()
            i += 1
        
        # création de l'array qui va stocker les OperationNodes
        operation_nodes = []

        i = 0
        # power operator
        while i < len(calc):
            if calc[i].token.value == "^": calc = calc[:i-1] + [OperatorNode(calc[i].token,calc[i-1],calc[i+1])] + calc[i+2:]
            else: i += 1
        i = 0
        while i < len(calc): # sum dif
            if calc[i].token.value in ("*","/"):
                calc = calc[:i-1] + [OperatorNode(calc[i].token,calc[i-1],calc[i+1])] + calc[i+2:]
            else: i += 1
        i = 0
        while i < len(calc): # sum dif
            if calc[i].token.value in ("+","-"):
                calc = calc[:i-1] + [OperatorNode(calc[i].token,calc[i-1],calc[i+1])] + calc[i+2:]
            else: i += 1
        i = 0
        while i < len(calc): # sum dif
            if calc[i].token.value in ("==","!=",">","<",">=","<="):
                calc = calc[:i-1] + [OperatorNode(calc[i].token,calc[i-1],calc[i+1])] + calc[i+2:]
            else: i += 1
        return(calc[0])


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
