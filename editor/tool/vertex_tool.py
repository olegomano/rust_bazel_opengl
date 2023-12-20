import sprite

class VertexTool:
    ID = "VertexTool"
    ICON = "editor/icons/actions/vertex_tool.png"
    ACTION = "action_vertex_tool"

    def __init__(self,sprite_manager : sprite.SpriteManager):
        self._sprite_manager = sprite_manager
    
    def handle_mouse_event(self,event):
        print(event)

    def select(self):
        pass

    def un_select(self):
        pass
