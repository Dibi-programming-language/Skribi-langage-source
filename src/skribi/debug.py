
def debug_print(instance, name="Debug", first_prefix="", second_prefix="", base = None):
    if base == instance:
        print(first_prefix, name, "= Debug tree root")
    else:
        if base == None:
            base = instance
        try:
            dico = instance.__dict__
            print(first_prefix, name, '('+instance.__class__.__name__+')')
            keys = tuple(dico.keys())
            len_keys = len(keys)
            i = 0
            while i < len_keys-1:
                debug_print(dico[keys[i]], keys[i], second_prefix+" ├──", second_prefix+" │  ", base)
                i += 1
            debug_print(dico[keys[i]], keys[i], second_prefix+" └──", second_prefix+"    ", base)
        except:
            print(first_prefix, name, '=', str((instance,))[1:-2])





