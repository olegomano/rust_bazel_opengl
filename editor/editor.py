from PyQt5 import uic
from PyQt5.QtWidgets import QApplication, QWidget, QMainWindow,QFileDialog,QGraphicsScene, QGraphicsPixmapItem
from PyQt5.QtGui import QImage, QPixmap, QIcon
from os import listdir
from os.path import isfile, join
import sprite
from tool import mesh_tool
from tool import vertex_tool

Form, Base = uic.loadUiType("editor/ui/main.ui")

class MainWindow(Base, Form):
    def __init__(self, parent=None):
        super(self.__class__, self).__init__(parent)
        self._scene = QGraphicsScene(self)
        
        self.setupUi(self)

        self.action_open.triggered.connect(lambda: self.open_image_selector())
        self.graphicsView.setScene(self._scene)
        self.graphicsView.mousePressEvent = lambda event : self.handle_mouse_event(event)
        self.action_add_sprite.triggered.connect(lambda : self.add_sprite())
        
        self._sprite_manager = sprite.SpriteManager()
        self._sprite_widget = sprite.SpriteTreeWidget(
            self.treeWidget,
            self.tableWidget,
            self._sprite_manager,
        )

        self._tools = {
            mesh_tool.MeshTool.ID : mesh_tool.MeshTool(self._sprite_manager),
            vertex_tool.VertexTool.ID : vertex_tool.VertexTool(self._sprite_manager),
        }
        self._selected_tool = None
        self.init_tools()
    
    def handle_mouse_event(self,event):
        if self._selected_tool is not None:
            self._selected_tool.handle_mouse_event(event)

    def add_sprite(self):
        self._sprite_manager.AddSprite(selected = True)

    #iterate over all the actions in the UI and set their icons if 
    #they are available
    def init_tools(self):
        for _,tool in self._tools.items():
            action = getattr(self,tool.ACTION)
            action.setIcon(QIcon(tool.ICON))
            action.toggled.connect(lambda checked,tool=tool : self.select_tool(tool,checked))
    
    def select_tool(self,tool,selected):
        print("selected tool {} {}".format(str(tool),str(selected)))
        if selected:
            self._selected_tool = tool
            for _,d_tool in self._tools.items():
                if d_tool is not tool:
                    print("Setting tool {} to false".format(d_tool))
                    getattr(self,d_tool.ACTION).setChecked(False)

    def open_image_selector(self):
        fname = QFileDialog.getOpenFileName(
                self, 
                'Open file',
                '/home/oleg/Documents/Dev/Galaga/assets/image',
                "Image files (*.png *.gif)"
        )

        print(fname)
        self.display_image(fname[0])

    def display_image(self,path):
        image = QImage(path)
        graphics_item = QGraphicsPixmapItem(QPixmap.fromImage(image))
        self._scene.addItem(graphics_item)
    

if __name__ == "__main__":
    app = QApplication([])
    main_window = MainWindow()
    main_window.show()
    app.exec()
