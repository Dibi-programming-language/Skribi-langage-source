from variables_and_types import *
from custom_exception import *


# ================================== #
# element that can contain variables #
# ================================== #

class ContainVariables:

    def __init__(self, parent=None):
        self.parent = parent
        self.variables = {}

    def set_variable(self, name: str, variable):
        self.variables[name] = variable

    def get_variable(self, name: str, current_scope):
        if name in self.variables:
            return self.variables[name]
        elif self.parent:
            return self.parent.get_variable(name)
        else:
            return SkribiException("Variable '{}' not found".format(name), "interpreter", current_scope.trace())


# ====================== #
# class of a Skribi file #
# ====================== #

class SkribiFile:
    def __init__(self, content, path):
        self.content = content
        self.path = path

    def set_content(self, content):
        self.content = content

    def get_content(self):
        return self.content

    def get_path(self):
        return self.path
