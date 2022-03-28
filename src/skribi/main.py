# ====================== #
# Skribi language module #
# ====================== #

# instance of Program class
from src.skribi import program, skribi_file

program_instance = program.Program()

# shell's file
shell_file = skribi_file.SkribiFile(None)


def execute(code, file):
    """ Execute code in Skribi """
    print("Executing code in Skribi...")
    if file:
        program_instance.analyse(skribi_file.SkribiFile(code))
    else:
        shell_file.set_content(code)
        program_instance.analyse(shell_file)
