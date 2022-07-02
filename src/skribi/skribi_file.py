#!/usr/bin/env python3
# *-* coding:utf-8 *-*

from src.skribi.variables_and_types import *
from src.skribi.custom_exception import *


# ================================== #
# element that can contain variables #
# ================================== #

class ContainsVariables:

    def __init__(self, parent=None):
        self.parent = parent
        self.variables = {}

    def set_variable(self, name: str, variable, current_scope) -> SkribiException or None:
        b = self.check_and_set_variable_in_parent(name, variable, current_scope)
        if isinstance(b, SkribiException):
            return b
        elif not b:
            # variable not exist, return an error
            return SkribiException("Variable '{}' not found, please create it before use it".format(name),
                                   "interpreter", current_scope.trace())

    def get_variable(self, name: str, current_scope):
        if name in self.variables:
            return self.variables[name]
        elif self.parent:
            return self.parent.get_variable(name)
        else:
            return SkribiException("Variable '{}' not found".format(name), "interpreter", current_scope.trace())

    def check_and_set_variable_in_parent(self, name: str, variable, current_scope) -> bool or SkribiException:
        if name in self.variables:
            if self.variables[name].type != variable.type:
                return SkribiException("Variable '{}' already exists with different type".format(name), "interpreter",
                                       current_scope.trace())
            else:
                self.variables[name] = variable
                return True
        elif self.parent:
            return self.parent.check_and_set_variable_in_parent(name, variable, current_scope)
        else:
            return False

    def check_name(self, name: str) -> SkribiException or None:
        if name in self.variables:
            return True
        elif self.parent:
            return self.parent.check_name(name)
        else:
            return False

    def create_variable(self, name: str, variable, current_scope) -> SkribiException or None:
        if name in self.variables:
            return SkribiException("Variable '{}' already exists".format(name), "interpreter", current_scope.trace())
        else:
            self.variables[name] = variable
            return None


# ============================== #
# element that can contain types #
# ============================== #

class ContainsTypes:

    def __init__(self, parent=None):
        self.parent = parent
        self.types = {}

    def set_type(self, name: str, type_, current_scope) -> SkribiException or None:
        b = self.check_and_set_type_in_parent(name, type_, current_scope)
        if isinstance(b, SkribiException):
            return b
        elif not b:
            # type not exist, return an error
            return SkribiException("Type '{}' not found, please create it before use it".format(name), "interpreter",
                                   current_scope.trace())

    def get_type(self, name: str, current_scope):
        if name in self.types:
            return self.types[name]
        elif self.parent:
            return self.parent.get_type(name)
        else:
            return SkribiException("Type '{}' not found".format(name), "interpreter", current_scope.trace())

    def check_and_set_type_in_parent(self, name: str, type_, current_scope) -> bool or SkribiException:
        if name in self.types:
            if self.types[name].type != type_.type:
                return SkribiException("Type '{}' already exists with different type".format(name), "interpreter",
                                       current_scope.trace())
            else:
                self.types[name] = type_
                return True
        elif self.parent:
            return self.parent.check_and_set_type_in_parent(name, type_, current_scope)
        else:
            return False

    def check_name(self, name: str) -> SkribiException or None:
        if name in self.types:
            return True
        elif self.parent:
            return self.parent.check_name(name)
        else:
            return False

    def create_type(self, name: str, type_, current_scope) -> SkribiException or None:
        if name in self.types:
            return SkribiException("Type '{}' already exists".format(name), "interpreter", current_scope.trace())
        else:
            self.types[name] = type_
            return None


# ====================== #
# class of a Skribi file #
# ====================== #

class SkribiFile(ContainsVariables, ContainsTypes):
    def __init__(self, content, path):
        super().__init__()
        self.content = content
        self.path = path
        self.lexer = None
        self.parser = None
        self.result = None

    def set_content(self, content):
        self.content = content

    def get_content(self):
        return self.content

    def get_path(self):
        return self.path


# ====================== #
# class of a scope stack #
# ====================== #

class ScopeStack:
    def __init__(self):
        self.stack = []

    def push(self, scope):
        self.stack.append(scope)

    def pop(self):
        return self.stack.pop()

    def get_current_scope(self):
        return self.stack[-1]

    def get_trace(self):
        for scope in self.stack:
            yield scope.trace()
