# ====================== #
# class of a Skribi file #
# ====================== #

class SkribiFile:
    def __init__(self, content):
        self.content = content

    def set_content(self, content):
        self.content = content

    def get_content(self):
        return self.content
