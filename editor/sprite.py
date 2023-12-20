from dataclasses import dataclass,field
from typing import List
from PyQt5.QtWidgets import QApplication, QTreeWidget, QTreeWidgetItem, QVBoxLayout, QWidget, QTableWidgetItem

@dataclass
class Attribute:
    name : str
    vertex_id : int

@dataclass
class Vertex:
    sprite_id : int
    vertex_id : int
    uv_x : float
    uv_y : float
    vertex_list : dict[str,Attribute] =  field(default_factory=lambda: {})

@dataclass
class Sprite:
    id : int
    vertex_list : dict[int,Vertex] =  field(default_factory=lambda: {})
    
    def AddVertex(self,id,x,y):
        self.vertex_list[id] = Vertex(
            vertex_id = id,
            sprite_id = self.id,
            uv_x = x,
            uv_y = y,
        )


class SpriteManager:
    def __init__(self):
        self._id = 0
        self._sprite_list = {}
        self._selected = None
        self._on_new_sprite = None
        self._on_vertex_added = None
        
        self._on_vertex_selected = None
        self._on_sprite_selected = None

    def AddSprite(self, selected : bool = False) -> int: 
        id = self._id
        self._id = self._id + 1
        self._sprite_list[id] = Sprite(id=id)
        if selected:
            self._selected = id
        if self._on_new_sprite is not None:
            print("Created new sprite {}".format(id))
            self._on_new_sprite(id)
        return id

    def AddVertex(self,x : float, y : float):
        id = self._id
        self._id = self._id + 1
        self._sprite_list[self._selected].AddVertex(id,x,y)
        self._on_vertex_added(self._selected,id)
        
    def SelectSprite(self, id : int):
        self._selected = id
        print("Selected sprite {}".format(id))
        if self._on_sprite_selected is not None:
            self._on_sprite_selected(id)
    
    def SelectId(self,id : int):
        if id in self._sprite_list:
            self.SelectSprite(id)
        else:
            pass


class SpriteTreeWidget:
    def __init__(self,tree_widget,table_widget,sprite_manager):
        self._tree_widget = tree_widget
        self._table_widget = table_widget

        self._tree_widget.itemSelectionChanged.connect(self.OnItemSelected)
        self._table_widget.itemChanged.connect(self.OnTableItemSelected)

        self._table_widget.setColumnCount(2)  # Set the number of columns
        self._table_widget.setRowCount(2)  # Set the number of rows
        self._table_widget.horizontalHeader().setStretchLastSection(True)

        self._sprite_manager = sprite_manager
        self._sprite_manager._on_new_sprite = self.OnSpriteCreated
        self._sprite_manager._on_vertex_added = self.OnVertexAdded
        self._sprite_manager_.on_vertex_selected = self.OnVertexSelected

        self._items = {}
    
    def OnSpriteCreated(self,sprite_id : int):
        item = QTreeWidgetItem(self._tree_widget)
        item.setText(0,str(sprite_id))
        self._items[sprite_id] = item
    
    def OnVertexAdded(self,sprite_id : int, vertex_id : int):
        item = QTreeWidgetItem(self._items[sprite_id])
        
        sprite = self._sprite_manager._sprite_list[sprite_id]
        vertex = sprite.vertex_list[vertex_id]

        item.setText(0,str(vertex_id))
        self._items[vertex_id] = item
        
        x_table_item = QTableWidgetItem("uv_x")
        self._table_widget.setItem(0, 0, x_table_item)
        x_table_item_value = QTableWidgetItem(str(vertex.uv_x))
        self._table_widget.setItem(0, 1, x_table_item_value)
        

        y_table_item = QTableWidgetItem("uv_y")
        self._table_widget.setItem(1, 0, y_table_item)

        y_table_item_value = QTableWidgetItem(str(vertex.uv_y))
        self._table_widget.setItem(1, 1, y_table_item_value)
    
    #callback from the SpriteManager on when a vertex is selected
    def OnVertexSelected(self,vertex_id : int):
        pass

    # callback from UI on when an item in the TreeWidget is selected
    def OnItemSelected(self):
        self._table_widget.clear()
        selected_item = self._tree_widget.selectedItems()[0]
        self._sprite_manager.SelectId(int(selected_item.text(0)))


    def OnTableItemSelected(self,item):
        pass
