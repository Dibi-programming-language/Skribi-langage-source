
def debug_print(instance, name="Debug", first_prefix="", second_prefix="", root = None, equality = " = "):
    if root == instance:
        print(first_prefix, name + equality + "Debug tree root")
    else:
        if root == None:
            root = instance
        if instance.__class__ in (list,tuple):
            simple = True
            for item in instance:
                if not type(item) in (int,float,bool):
                    simple = False
            if simple:
                print(first_prefix, name + equality + instance.__repr__())
            else:
                print(first_prefix, name, '('+instance.__class__.__name__+')')
                for i in range(len(instance)-1):
                    debug_print(instance[i], "", second_prefix+" ╠══", second_prefix+" ║  ", root, "")
                debug_print(instance[-1], "", second_prefix+" ╚══", second_prefix+"    ", root, "")
        else:
            try:
                dico = instance.__dict__
                print(first_prefix, name, '('+instance.__class__.__name__+')')
                keys = tuple(dico.keys())
                for key in keys[:-1]:
                    debug_print(dico[key], key, second_prefix+" ├──", second_prefix+" │  ", root)
                debug_print(dico[keys[-1]], keys[-1], second_prefix+" └──", second_prefix+"    ", root)
            except:
                print(first_prefix, name + equality + instance.__repr__())


class DebugCount:
    def __init__(self, name = "Debug") -> None:
        self.value = -1
        self.name = name
    
    # permet de compter dans la console afin de se repérer dans l'exécution d'un code
    def i(self, add = 1):
        self.value += add
        print(' '+self.name, self.value)


def debug(*values, name = "Debug"):
    print(' '+name, end=' ')
    for val in values:
        print(val, end=' ')
    print()



