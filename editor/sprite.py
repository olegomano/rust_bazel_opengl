from dataclasses import dataclass,field
from typing import List
from PyQt5.QtWidgets import (
    QApplication, 
    QTreeWidget, 
    QTreeWidgetItem,
    QVBoxLayout, 
    QWidget, 
    QTableWidgetItem, 
    QGraphicsScene, 
    QGraphicsView,
    QGraphicsPolygonItem,
    QGraphicsEllipseItem,
    QMenu,
    QAction,
)


from PyQt5.QtGui import QPolygonF,QColor
from PyQt5.QtCore import QPoint,QPointF
from PyQt5.QtCore import Qt

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
        vertex = Vertex(
            vertex_id = id,
            sprite_id = self.id,
            uv_x = x,
            uv_y = y,
        )
        self.vertex_list[id] = vertex
        return vertex


class SpriteManager:
    def __init__(self):
        self._id = 0
        
        self._sprite_list = {}
        self._vertex_list = {}

        self._selected = None
        self._on_new_sprite = None
        self._on_vertex_added = None
        
        self._on_vertex_selected = None
        self._on_sprite_selected = None
        self._on_vertex_deleted = None

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
        vertex = self._sprite_list[self._selected].AddVertex(id,x,y)
        self._vertex_list[id] = vertex
        self._on_vertex_added(self._selected,id)
    
    def SelectSprite(self, id : int):
        self._selected = id
        print("Selected sprite {}".format(id))
        if self._on_sprite_selected is not None:
            self._on_sprite_selected(id)
    
    def SelectVertex(self, id : int):
        print("Selected vertex {}".format(id))
        if self._on_vertex_selected is not None:
            self._on_vertex_selected(self._vertex_list[id].sprite_id,id)

    def SelectId(self,id : int):
        if id in self._sprite_list:
            self.SelectSprite(id)
        if id in self._vertex_list:
            self.SelectVertex(id)

    def DeleteId(self,id : int):
        print("Deleting id {}".format(id))
        if id in self._vertex_list:
            vertex = self._vertex_list[id]
            del self._vertex_list[id]
            del self._sprite_list[vertex.sprite_id].vertex_list[id]
            self._on_vertex_deleted(vertex.sprite_id,id)

class SpriteTreeWidget:
    MARKER_SIZE = 15
    
    def __init__(self,tree_widget,table_widget,graphics_scene, sprite_manager):
        self._tree_widget = tree_widget
        self._table_widget = table_widget
        self._graphics_scene = graphics_scene

        self._tree_widget.itemSelectionChanged.connect(self.OnItemSelected)
        self._table_widget.itemChanged.connect(self.OnTableItemSelected)

        self._table_widget.setColumnCount(2)  # Set the number of columns
        self._table_widget.setRowCount(2)  # Set the number of rows
        self._table_widget.horizontalHeader().setStretchLastSection(True)

        self._sprite_manager = sprite_manager
        self._sprite_manager._on_new_sprite = self.OnSpriteCreated
        self._sprite_manager._on_vertex_added = self.OnVertexAdded
        self._sprite_manager._on_vertex_selected = self.OnVertexSelected
        self._sprite_manager._on_vertex_deleted = self.DeleteVertex

        #set up right click handler on tree widget
        self._tree_widget.setContextMenuPolicy(Qt.CustomContextMenu)
        self._tree_widget.customContextMenuRequested.connect(self.ShowContextMenu)


        # sprite_id -> QTreeWidgetItem
        self._items = {}
        # sprite_id -> QPolygon
        self._sprite_scene_items = {}
        # vertex_id -> QEllipse
        self._vertex_markers = {}
        # marker for currently selected vertex
        self._selected_vertex_item = None
    
    #show the right click context menu on tree item
    def ShowContextMenu(self,point):
        def do_delete(id):
            return lambda: self._sprite_manager.DeleteId(id)
    

        item = self._tree_widget.itemAt(point)
        id = int(item.text(0))

        menu = QMenu(self._tree_widget)
        delete_action = QAction("Delete", self._tree_widget)
        delete_action.triggered.connect(do_delete(id))


        menu.addAction(delete_action)
        action = menu.exec_(self._tree_widget.mapToGlobal(point))

    def OnSpriteCreated(self,sprite_id : int):
        item = QTreeWidgetItem(self._tree_widget)
        item.setText(0,str(sprite_id))
        self._items[sprite_id] = item
    
    def OnVertexAdded(self,sprite_id : int, vertex_id : int):
        vertex = self._sprite_manager._vertex_list[vertex_id]

        item = QTreeWidgetItem(self._items[sprite_id])
        item.setText(0,str(vertex_id))
        self._items[vertex_id] = item

        if sprite_id not in self._sprite_scene_items:
            polygon_item = QGraphicsPolygonItem()
            polygon = QPolygonF()
            self._graphics_scene.addItem(polygon_item)
            self._sprite_scene_items[sprite_id] = polygon_item,polygon
        
        polygon_item, polygon = self._sprite_scene_items[sprite_id]
        polygon.append(QPointF(vertex.uv_x,vertex.uv_y)) 
        polygon_item.setPolygon(polygon)
        

        circle_item = QGraphicsEllipseItem(0,0,self.MARKER_SIZE,self.MARKER_SIZE)
        circle_item.setPos(vertex.uv_x - self.MARKER_SIZE / 2,vertex.uv_y- self.MARKER_SIZE/3)
        self._graphics_scene.addItem(circle_item)
        self._vertex_markers[vertex_id] = circle_item

        self.ShowVertexAttributes(vertex_id)
        



    def DeleteVertex(self, sprite_id : int, vertex_id : int):
        #delete vertex marker
        vertex_marker = self._vertex_markers[vertex_id]
        self._graphics_scene.removeItem(vertex_marker)
        #delete old polygon item
        polygon_item,_ = self._sprite_scene_items[sprite_id]
        self._graphics_scene.removeItem(polygon_item)
            
        #remove tree item from the tree widget
        self._items[sprite_id].removeChild(self._items[vertex_id])
        del self._items[vertex_id]
        
        #we re-create the polygon by iterating over all the vertex
        #we assume the vertex_id has already beem deleted out of the sprite_manager
        polygon_item = QGraphicsPolygonItem()
        polygon = QPolygonF()
        sprite = self._sprite_manager._sprite_list[sprite_id]
        for vertex_id,vertex in sprite.vertex_list.items():
            polygon.append(QPointF(vertex.uv_x,vertex.uv_y))

        polygon_item.setPolygon(polygon)
        self._graphics_scene.addItem(polygon_item)
        self._sprite_scene_items[sprite_id] = polygon_item,polygon


    
    def ShowVertexAttributes(self,vertex_id : int):
        vertex = self._sprite_manager._vertex_list[vertex_id]
        
        x_table_item = QTableWidgetItem( "uv_x")
        self._table_widget.setItem(0, 0, x_table_item)
        x_table_item_value = QTableWidgetItem(str(vertex.uv_x))
        self._table_widget.setItem(0, 1, x_table_item_value)
        y_table_item = QTableWidgetItem("uv_y")
        self._table_widget.setItem(1, 0, y_table_item)
        y_table_item_value = QTableWidgetItem(str(vertex.uv_y))
        self._table_widget.setItem(1, 1, y_table_item_value)

    #callback from the SpriteManager on when a vertex is selected
    def OnVertexSelected(self,sprite_id : int, vertex_id : int):
        self._table_widget.clear()
        self.ShowVertexAttributes(vertex_id)
        if self._selected_vertex_item is not None:
            self._graphics_scene.removeItem(self._selected_vertex_item)

        
        vertex = self._sprite_manager._vertex_list[vertex_id]

        circle_item = QGraphicsEllipseItem(0,0,self.MARKER_SIZE,self.MARKER_SIZE)
        circle_item.setPos(vertex.uv_x - self.MARKER_SIZE / 2,vertex.uv_y- self.MARKER_SIZE/3)
        circle_item.setBrush(QColor(255, 0, 0))
        self._graphics_scene.addItem(circle_item)
        
        self._selected_vertex_item = circle_item

    # callback from UI on when an item in the TreeWidget is selected
    def OnItemSelected(self):
        selected_item = self._tree_widget.selectedItems()[0]
        self._sprite_manager.SelectId(int(selected_item.text(0)))


    def OnTableItemSelected(self,item):
        pass
