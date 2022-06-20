# this file is the class for the type model and the class for custom types

class BaseType:
    """
    A base type is a type that can be used in a variable.
    """

    def __init__(self, name, scope, extends=None, is_primitive=False):
        self.name = name
        self.scope = scope
        self.extends = extends
        self.is_primitive = is_primitive
        self.scope.add_type(self)

    def __str__(self):
        return self.name

    def __repr__(self):
        return self.name

    def __eq__(self, other):
        return self.name == other.name

    def __hash__(self):
        return hash(self.name + str(self.scope))

    def extends_type(self, other) -> bool:
        """
        Returns True if this type extends the other type.
        :param other: another type
        :return:
        """

        if self.extends is None:
            return False

        if self.extends == other:
            return True

        return self.extends.extends_type(other)


# Custom types
class CustomType(BaseType):
    """
    A custom type is a type that is not defined in the language.
    """


    def __init__(self, name, scope, extends=None, is_primitive=False):
        super().__init__(name, scope, extends, is_primitive)

        self.fields = []
        self.methods = []

        self.scope.add_type(self)

        
primitives_types = []


class PrimitiveType(BaseType):
    """
    A primitive type is a type defined by the language.
    """

    def __init__(self, name, extends=None):
        super().__init__(name, None, is_primitive=True, extends=extends)
        primitives_types.append(self)


class Function:
    """
    A function has input and output types and a body with many nodes
    """
