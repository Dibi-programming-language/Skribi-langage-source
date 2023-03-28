
def debug_print(instance, name="Debug", first_prefix="", second_prefix="", root = None):
    if root == instance:
        print(first_prefix, name, "= Debug tree root")
    else:
        if root == None:
            root = instance
        try:
            dico = instance.__dict__
            print(first_prefix, name, '('+instance.__class__.__name__+')')
            keys = tuple(dico.keys())
            for key in keys:
                debug_print(dico[key], key, second_prefix+" ├──", second_prefix+" │  ", root)
            debug_print(dico[keys[i]], keys[i], second_prefix+" └──", second_prefix+"    ", root)
        except:
            print(first_prefix, name, '=', str((instance,))[1:-2])


class DebugCount:
    def __init__(self, name = "Debug") -> None:
        self.value = -1
        self.name = name
    
    def i(self, add = 1):
        self.value += add
        print(' '+self.name, self.value)


def debug(*values, name = "Debug"):
    print(' '+name, end=' ')
    for val in values:
        print(val, end=' ')
    print()



