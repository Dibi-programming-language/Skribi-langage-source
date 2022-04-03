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
