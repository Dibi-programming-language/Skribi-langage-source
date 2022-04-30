# ===================================== #
# exception for custom errors in skribi #
# ===================================== #

class ExceptionLine(Exception):
    def __init__(self, line, file):
        self.line = line
        self.file = file

    def __str__(self):
        return str(self.line) + " in " + self.file


class SkribiException(Exception):
    """
    Custom exception for skribi
    """

    # when = token / parse / execute; lines = list of lines
    def __init__(self, message, when, lines: list = None):
        self.message = message
        self.when = when
        self.lines = lines

    def __str__(self):
        return self.message

    def print_complete_error(self):
        print("Error", self.message)
        print("When " + self.when)
        if self.lines is not None:
            for line in self.lines:
                print("\t at: line", line)
