import sprite

class MeshTool:
    ID = "MeshTool"
    ICON = "editor/icons/actions/mesh_tool.png"
    ACTION = "action_mesh_tool"

    def __init__(self,sprite_manager : sprite.SpriteManager):
        self._sprite_manager = sprite_manager
    
    def handle_mouse_event(self,x,y):
        self._sprite_manager.AddVertex(x,y)


    def select(self):
        pass

    def un_select(self):
        pass
