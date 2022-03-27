# ====================== #
# Skribi language module #
# ====================== #

import program
import skribi_file

# instance of Program class
program_instance = program.Program()

# shell's file
shell_file = skribi_file.SkribiFile(None)


def execute(code, file):
    """ Execute code in Skribi """
    if file:
        program_instance.execute(skribi_file.SkribiFile(code))
    else:
        shell_file.set_content(code)
        program_instance.execute(shell_file)
