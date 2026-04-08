import sys
import re
import xml.etree.ElementTree as ET
from PyQt6.QtWidgets import (QApplication, QMainWindow, QGraphicsView,
                             QGraphicsScene, QGraphicsRectItem, QTreeWidget, QTreeWidgetItem,
                             QHBoxLayout, QWidget, QVBoxLayout, QLabel,
                             QTableWidget, QTableWidgetItem, QPushButton, QFileDialog, QMessageBox)
from PyQt6.QtCore import Qt, QPointF
from PyQt6.QtGui import QColor, QPen, QBrush


# --- 1. ELEMENT & GRID LOGIC ---

class DesignerElement(QGraphicsRectItem):
    def __init__(self, tool_data, x, y, template):
        # Convert ordered list back to dict for easy lookup during init
        arg_dict = dict(tool_data['args'])
        w = int(arg_dict.get('width', 100))
        h = int(arg_dict.get('height', 40))
        super().__init__(0, 0, w, h)

        self.template = template
        self.tool_data = tool_data
        # We keep properties as a dict for the UI table, but tool_data['args'] defines the order
        self.properties = arg_dict.copy()

        self.setPos(x, y)
        self.setFlags(self.GraphicsItemFlag.ItemIsMovable |
                      self.GraphicsItemFlag.ItemIsSelectable |
                      self.GraphicsItemFlag.ItemSendsGeometryChanges)
        self.setPen(QPen(QColor("#00ffcc"), 2))
        self.setBrush(QBrush(QColor(0, 255, 204, 40)))

    def itemChange(self, change, value):
        if change == self.GraphicsItemChange.ItemPositionChange and self.template:
            gs = self.template.grid_size
            new_pos = value
            x = max(0, round(new_pos.x() / gs) * gs)
            y = max(0, round(new_pos.y() / gs) * gs)
            return QPointF(x, y)
        return super().itemChange(change, value)


class LayoutZone(QGraphicsRectItem):
    def __init__(self, name, x, y, w, h, color_hex):
        super().__init__(0, 0, w, h)
        self.setPos(x, y)
        col = QColor(int(color_hex, 16))
        col.setAlpha(60)
        self.setBrush(QBrush(col))
        self.setPen(QPen(Qt.GlobalColor.darkGray, 1, Qt.PenStyle.DashLine))
        self.setZValue(-100)
        self.setFlag(self.GraphicsItemFlag.ItemIsSelectable, False)


# --- 2. MAIN EDITOR ---

class PixelDesignerApp(QMainWindow):
    def __init__(self, xml_path):
        super().__init__()
        self.setWindowTitle("UEFI Pixel Designer")
        self.resize(1400, 900)

        try:
            self.xml_root = ET.parse(xml_path).getroot()
        except Exception as e:
            QMessageBox.critical(self, "Error", f"Could not load {xml_path}: {e}")
            sys.exit(1)

        self.grid_size = 20
        self.init_ui()
        self.load_zones()

    def init_ui(self):
        central = QWidget()
        self.setCentralWidget(central)
        layout = QHBoxLayout(central)

        # Left: Toolbox
        self.toolbox = QTreeWidget()
        self.toolbox.setHeaderLabel("Components")
        for cat in self.xml_root.findall('Category'):
            parent = QTreeWidgetItem(self.toolbox, [cat.get('name')])
            for t in cat.findall('Tool'):
                child = QTreeWidgetItem(parent, [t.get('name')])

                # Store args as a list of tuples (name, default) to preserve XML order
                ordered_args = []
                for a in t.findall('Arg'):
                    ordered_args.append((a.get('name'), a.get('default')))

                child.setData(0, Qt.ItemDataRole.UserRole, {
                    'func': t.get('func'),
                    'args': ordered_args
                })

        self.toolbox.itemDoubleClicked.connect(self.add_element_from_tool)
        layout.addWidget(self.toolbox, 1)

        # Center: Canvas
        self.scene = QGraphicsScene(0, 0, 1920, 1080)
        self.view = QGraphicsView(self.scene)
        self.view.setBackgroundBrush(QBrush(QColor(33, 33, 33)))
        self.view.setFocusPolicy(Qt.FocusPolicy.StrongFocus)
        self.scene.selectionChanged.connect(self.sync_properties)
        layout.addWidget(self.view, 4)

        # Right: Properties & Buttons
        right_panel = QVBoxLayout()
        self.props_table = QTableWidget(0, 2)
        self.props_table.setHorizontalHeaderLabels(["Property", "Value"])
        self.props_table.itemChanged.connect(self.update_element_from_table)

        btn_import = QPushButton("Import from Rust (.rs)")
        btn_import.clicked.connect(self.import_from_rust)

        btn_export = QPushButton("Export to Rust (.rs)")
        btn_export.clicked.connect(self.export_to_rust)

        btn_del = QPushButton("Delete Selected")
        btn_del.clicked.connect(self.delete_selected)

        right_panel.addWidget(QLabel("Properties"))
        right_panel.addWidget(self.props_table)
        right_panel.addWidget(btn_del)
        right_panel.addWidget(btn_import)
        right_panel.addWidget(btn_export)
        layout.addLayout(right_panel, 1)

    def load_zones(self):
        """

        """
        template = self.xml_root.find('Template')
        if template is not None:
            layout = template.find('Layout')
            if layout is not None:
                for zone in layout.findall('Zone'):
                    z = LayoutZone(zone.get('name'), int(zone.get('x')), int(zone.get('y')),
                                   int(zone.get('w')), int(zone.get('h')), zone.get('color'))
                    self.scene.addItem(z)

    # --- ACTION HANDLERS ---

    def keyPressEvent(self, event):
        """

        :param event:
        """
        if event.key() == Qt.Key.Key_Delete:
            self.delete_selected()

    def delete_selected(self):
        """

        :return:
        """
        items = self.scene.selectedItems()
        if not items: return
        for item in items:
            if isinstance(item, DesignerElement):
                self.scene.removeItem(item)
        self.props_table.setRowCount(0)

    def add_element_from_tool(self, item):
        """

        :param item:
        """
        data = item.data(0, Qt.ItemDataRole.UserRole)
        if data:
            el = DesignerElement(data, 100, 100, self)
            self.scene.addItem(el)

    def sync_properties(self):
        """

        """
        self.props_table.blockSignals(True)
        self.props_table.setRowCount(0)
        sel = self.scene.selectedItems()
        if sel and isinstance(sel[0], DesignerElement):
            obj = sel[0]
            props = [("x", int(obj.x())), ("y", int(obj.y()))]
            # Add other properties in order
            for name, _ in obj.tool_data['args']:
                props.append((name, obj.properties.get(name, "")))

            self.props_table.setRowCount(len(props))
            for i, (k, v) in enumerate(props):
                self.props_table.setItem(i, 0, QTableWidgetItem(k))
                self.props_table.setItem(i, 1, QTableWidgetItem(str(v)))
        self.props_table.blockSignals(False)

    def update_element_from_table(self, item):
        """

        :param item:
        :return:
        """
        if item.column() != 1: return
        sel = self.scene.selectedItems()
        if not sel: return
        obj = sel[0]
        key, val = self.props_table.item(item.row(), 0).text(), item.text()

        if key == 'x':
            obj.setX(float(val))
        elif key == 'y':
            obj.setY(float(val))
        else:
            obj.properties[key] = val
            if key in ['width', 'height']:
                r = obj.rect()
                if key == 'width':
                    r.setWidth(float(val))
                else:
                    r.setHeight(float(val))
                obj.setRect(r)

    # --- RUST INTEROP ---

    def find_tool_by_func(self, func_name):
        """

        :param func_name:
        :return:
        """
        for cat in self.xml_root.findall('Category'):
            for t in cat.findall('Tool'):
                if t.get('func') == func_name:
                    return {
                        'func': t.get('func'),
                        'name': t.get('name'),
                        'args': [(a.get('name'), a.get('default')) for a in t.findall('Arg')]
                    }
        return None

    def export_to_rust(self):
        """

        :return:
        """
        path, _ = QFileDialog.getSaveFileName(self, "Export Rust UI", "", "Rust (*.rs)")
        if not path: return
        with open(path, 'w') as f:
            f.write("use crate::pixel_graphics::PixelGraphics;\n\npub fn render_ui(pg: &mut PixelGraphics) {\n")
            items = [i for i in self.scene.items() if isinstance(i, DesignerElement)]
            for item in sorted(items, key=lambda i: (i.y(), i.x())):
                # Always start with x, y as standard for the module
                args = [str(int(item.x())), str(int(item.y()))]
                # Append remaining arguments in the order specified in the tool data
                for name, _ in item.tool_data['args']:
                    val = item.properties.get(name)
                    if "0x" in str(val) or str(val).lower() in ['true', 'false']:
                        args.append(str(val).lower())
                    else:
                        args.append(f'"{val}"' if any(c.isalpha() for c in str(val)) else str(val))
                f.write(f"    pg.{item.tool_data['func']}({', '.join(args)});\n")
            f.write("}\n")

    def import_from_rust(self):
        """

        :return:
        """
        path, _ = QFileDialog.getOpenFileName(self, "Import Rust UI", "", "Rust (*.rs)")
        if not path: return
        with open(path, 'r') as f:
            content = f.read()

        pattern = r"pg\.(\w+)\s*\((.*?)\)\s*;"
        matches = re.findall(pattern, content, re.DOTALL)

        for func_name, args_raw in matches:
            args = [a.strip().strip('"') for a in re.split(r',(?=(?:[^"]*"[^"]*")*[^"]*$)', args_raw)]
            tool = self.find_tool_by_func(func_name)
            if tool and len(args) >= 2:
                try:
                    x, y = int(args[0]), int(args[1])
                    el = DesignerElement(tool, x, y, self)
                    # Map imported args to the tool's ordered keys (starting from index 2)
                    for i, val in enumerate(args[2:]):
                        if i < len(tool['args']):
                            key = tool['args'][i][0]
                            el.properties[key] = val
                    self.scene.addItem(el)
                except:
                    continue


if __name__ == "__main__":
    app = QApplication(sys.argv)
    ex = PixelDesignerApp("definitions.xml")
    ex.show()
    sys.exit(app.exec())