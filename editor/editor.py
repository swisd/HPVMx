import sys
import re
import xml.etree.ElementTree as ET
from xml.dom import minidom
from PyQt6.QtWidgets import (QApplication, QMainWindow, QGraphicsView,
                             QGraphicsScene, QGraphicsRectItem, QTreeWidget, QTreeWidgetItem,
                             QHBoxLayout, QWidget, QVBoxLayout, QLabel,
                             QTableWidget, QTableWidgetItem, QPushButton, QFileDialog, QMessageBox, QTabWidget, QHeaderView)
from PyQt6.QtCore import Qt, QPointF
from PyQt6.QtGui import QColor, QPen, QBrush


# --- 1. ELEMENT & GRID LOGIC ---

class DesignerElement(QGraphicsRectItem):
    def __init__(self, tool_data, x, y, parent_app):
        # Initialize basic attributes
        self.preview_mode = False
        self.tool_data = tool_data
        self.parent_app = parent_app

        # Tool data args is a dict: {'width': '100', 'height': '50', ...}
        self.properties = tool_data.get('args', {}).copy()

        w = int(self.properties.get('width', 100))
        h = int(self.properties.get('height', 40))
        super().__init__(0, 0, w, h)

        self.setPos(x, y)
        self.setFlags(self.GraphicsItemFlag.ItemIsMovable |
                      self.GraphicsItemFlag.ItemIsSelectable |
                      self.GraphicsItemFlag.ItemSendsGeometryChanges)

        self.editor_pen = QPen(QColor("#00ffcc"), 2)
        self.editor_brush = QBrush(QColor(0, 255, 204, 40))
        self.setPen(self.editor_pen)
        self.setBrush(self.editor_brush)

    def itemChange(self, change, value):
        if change == self.GraphicsItemChange.ItemPositionChange and self.parent_app:
            gs = self.parent_app.grid_size
            new_pos = value
            x = max(0, round(new_pos.x() / gs) * gs)
            y = max(0, round(new_pos.y() / gs) * gs)
            return QPointF(x, y)
        return super().itemChange(change, value)

    def paint(self, painter, option, widget=None):
        if not self.preview_mode:
            # DESIGNER MODE
            painter.setPen(QPen(QColor("#00ffcc"), 1, Qt.PenStyle.DashLine))
            painter.setBrush(QBrush(QColor(0, 255, 204, 20)))
            painter.drawRect(self.rect())

            painter.setPen(QColor("#00ffcc"))
            tag = self.tool_data.get('xml_tag') or "Element"
            painter.drawText(self.rect().adjusted(5, 2, 0, 0), Qt.AlignmentFlag.AlignTop, str(tag))
        else:
            # PREVIEW MODE
            tag = self.tool_data.get('xml_tag')
            painter.setPen(QPen(QColor("#ffffff"), 1))
            painter.setBrush(QBrush(QColor("#1a1a1a")))

            if tag == "FillRect":
                color = QColor(self.properties.get('color', '#444444').replace("0x", "#"))
                painter.setBrush(QBrush(color))
                painter.drawRect(self.rect())
            elif tag in ["Button", "Label", "LCDNumber"]:
                painter.drawRect(self.rect())
                text = self.properties.get('text') or self.properties.get('value', "")
                painter.drawText(self.rect(), Qt.AlignmentFlag.AlignCenter, str(text))
            else:
                painter.drawRect(self.rect())


class LayoutZone(QGraphicsRectItem):
    def __init__(self, name, x, y, w, h, color_hex):
        super().__init__(0, 0, w, h)
        self.setPos(x, y)
        try:
            col = QColor(color_hex.replace("0x", "#"))
        except:
            col = QColor(Qt.GlobalColor.gray)
        col.setAlpha(40)
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
        self.grid_size = 20

        try:
            self.xml_root = ET.parse(xml_path).getroot()
        except Exception as e:
            QMessageBox.critical(self, "Error", f"Could not load {xml_path}: {e}")
            sys.exit(1)

        self.init_ui()
        self.load_zones()

    def init_ui(self):
        central = QWidget()
        self.setCentralWidget(central)
        main_layout = QHBoxLayout(central)

        # Left: Toolbox
        self.toolbox = QTreeWidget()
        self.toolbox.setHeaderLabel("Components")
        for cat in self.xml_root.findall('Category'):
            parent = QTreeWidgetItem(self.toolbox, [cat.get('name')])
            for t in cat.findall('Tool'):
                child = QTreeWidgetItem(parent, [t.get('name')])
                tool_dat = {
                    "name": t.get('name'),
                    "func": t.get('func'),
                    "xml_tag": t.get('xml_name'),
                    "args": {a.get('name'): a.get('default') for a in t.findall('Arg')},
                    "arg_types": {a.get('name'): a.get('type') for a in t.findall('Arg')}
                }
                child.setData(0, Qt.ItemDataRole.UserRole, tool_dat)

        # Center: Tabs (Designer vs Preview)
        self.tabs = QTabWidget()
        self.scene = QGraphicsScene(0, 0, 1280, 800)

        # View 1: Designer
        self.view = QGraphicsView(self.scene)
        self.view.setBackgroundBrush(QBrush(QColor(33, 33, 33)))
        self.scene.selectionChanged.connect(self.sync_properties)

        # View 2: Preview
        self.preview_view = QGraphicsView(self.scene)
        self.preview_view.setBackgroundBrush(QBrush(QColor(0, 0, 0)))

        self.tabs.addTab(self.view, "Layout Designer")
        self.tabs.addTab(self.preview_view, "Live Preview")
        self.tabs.currentChanged.connect(self.on_tab_changed)

        # Right: Properties
        right_panel = QVBoxLayout()
        self.props_table = QTableWidget(0, 2)
        self.props_table.setHorizontalHeaderLabels(["Property", "Value"])
        self.props_table.horizontalHeader().setSectionResizeMode(QHeaderView.ResizeMode.Stretch)
        self.props_table.itemChanged.connect(self.update_element_from_table)

        btn_export_xml = QPushButton("Export to XML (.xml)")
        btn_export_xml.clicked.connect(self.export_to_xml)

        btn_export_rs = QPushButton("Export to Rust (.rs)")
        btn_export_rs.clicked.connect(self.export_to_rust)

        btn_del = QPushButton("Delete Selected")
        btn_del.clicked.connect(self.delete_selected)

        right_panel.addWidget(QLabel("Properties"))
        right_panel.addWidget(self.props_table)
        right_panel.addWidget(btn_del)

        btn_import = QPushButton("Import from Rust (.rs)")
        btn_import.clicked.connect(self.import_from_rust)
        right_panel.addWidget(btn_import)
        right_panel.addWidget(btn_export_xml)
        right_panel.addWidget(btn_export_rs)

        main_layout.addWidget(self.toolbox, 1)
        main_layout.addWidget(self.tabs, 4)
        main_layout.addLayout(right_panel, 1)

        self.toolbox.itemDoubleClicked.connect(self.add_element_from_tool)

    def load_zones(self):
        template = self.xml_root.find('Template')
        if template is not None:
            layout = template.find('Layout')
            if layout is not None:
                for zone in layout.findall('Zone'):
                    z = LayoutZone(zone.get('name'), int(zone.get('x')), int(zone.get('y')),
                                   int(zone.get('w')), int(zone.get('h')), zone.get('color'))
                    self.scene.addItem(z)

    def on_tab_changed(self, index):
        is_preview = (index == 1)
        if is_preview: self.scene.clearSelection()
        for item in self.scene.items():
            if isinstance(item, DesignerElement):
                item.preview_mode = is_preview
                item.setFlag(item.GraphicsItemFlag.ItemIsMovable, not is_preview)
                item.setFlag(item.GraphicsItemFlag.ItemIsSelectable, not is_preview)
                item.update()

    def add_element_from_tool(self, item):
        data = item.data(0, Qt.ItemDataRole.UserRole)
        if data:
            el = DesignerElement(data, 100, 100, self)
            self.scene.addItem(el)

    def sync_properties(self):
        self.props_table.blockSignals(True)
        self.props_table.setRowCount(0)
        sel = self.scene.selectedItems()

        if sel and isinstance(sel[0], DesignerElement):
            obj = sel[0]
            props = [
                ("INTERNAL_TAG", obj.tool_data.get('xml_tag', 'N/A')),
                ("INTERNAL_FUNC", obj.tool_data.get('func', 'N/A')),
                ("x", str(int(obj.x()))),
                ("y", str(int(obj.y())))
            ]

            for name, val in obj.properties.items():
                props.append((name, str(val)))

            self.props_table.setRowCount(len(props))
            for i, (k, v) in enumerate(props):
                k_item, v_item = QTableWidgetItem(k), QTableWidgetItem(v)
                if k.startswith("INTERNAL_"):
                    k_item.setFlags(Qt.ItemFlag.ItemIsEnabled)
                    v_item.setFlags(Qt.ItemFlag.ItemIsEnabled)
                self.props_table.setItem(i, 0, k_item)
                self.props_table.setItem(i, 1, v_item)

        self.props_table.blockSignals(False)

    def update_element_from_table(self, item):
        if item.column() != 1: return
        sel = self.scene.selectedItems()
        if not sel: return
        obj = sel[0]
        key = self.props_table.item(item.row(), 0).text()
        val = item.text()

        if key == 'x':
            obj.setX(float(val))
        elif key == 'y':
            obj.setY(float(val))
        elif not key.startswith("INTERNAL_"):
            obj.properties[key] = val
            if key in ['width', 'height']:
                r = obj.rect()
                if key == 'width':
                    r.setWidth(float(val))
                else:
                    r.setHeight(float(val))
                obj.setRect(r)

    def delete_selected(self):
        for item in self.scene.selectedItems():
            if isinstance(item, DesignerElement):
                self.scene.removeItem(item)
        self.props_table.setRowCount(0)

    def export_to_xml(self):
        path, _ = QFileDialog.getSaveFileName(self, "Save XML", "", "XML (*.xml)")
        if not path: return
        root = ET.Element("Window", {"width": "1920", "height": "1080"})
        frame = ET.SubElement(root, "Frame", {"name": "main_frame"})
        for item in self.scene.items():
            if isinstance(item, DesignerElement):
                tag = item.tool_data.get('xml_tag')
                if tag:
                    attrs = {"x": str(int(item.x())), "y": str(int(item.y()))}
                    attrs.update({k: str(v) for k, v in item.properties.items()})
                    ET.SubElement(frame, tag, attrs)

        xml_str = minidom.parseString(ET.tostring(root)).toprettyxml(indent="    ")
        with open(path, "w") as f:
            f.write(xml_str)

    def export_to_rust(self):
        path, _ = QFileDialog.getSaveFileName(self, "Save Rust", "", "Rust (*.rs)")
        if not path: return
        with open(path, 'w') as f:
            f.write("pub fn render_ui(pg: &mut PixelGraphics) {\n")
            items = sorted([i for i in self.scene.items() if isinstance(i, DesignerElement)],
                           key=lambda i: (i.y(), i.x()))
            for item in items:
                func = item.tool_data.get('func')
                args = [str(int(item.x())), str(int(item.y()))]
                for k, v in item.properties.items():
                    args.append(f'"{v}"' if item.tool_data['arg_types'].get(k) == 'string' else str(v))
                f.write(f"    pg.{func}({', '.join(args)});\n")
            f.write("}\n")

    def find_tool_by_func(self, func_name):
        """
        Scans the definitions to find a tool that matches the Rust function name.
        """
        for cat in self.xml_root.findall('Category'):
            for t in cat.findall('Tool'):
                if t.get('func') == func_name:
                    return {
                        "name": t.get('name'),
                        "func": t.get('func'),
                        "xml_tag": t.get('xml_name'),
                        "args": {a.get('name'): a.get('default') for a in t.findall('Arg')},
                        "arg_types": {a.get('name'): a.get('type') for a in t.findall('Arg')}
                    }
        return None

    def import_from_rust(self):
        """
        Parses a .rs file and spawns DesignerElements on the canvas.
        """
        path, _ = QFileDialog.getOpenFileName(self, "Import Rust UI", "", "Rust (*.rs)")
        if not path:
            return

        with open(path, 'r') as f:
            content = f.read()

        # Regex to find pg.function_name(arg1, arg2, ...);
        # It handles multi-line calls and ignores extra whitespace.
        pattern = r"pg\.(\w+)\s*\((.*?)\)\s*;"
        matches = re.findall(pattern, content, re.DOTALL)

        if not matches:
            QMessageBox.warning(self, "Import", "No valid 'pg.' function calls found in file.")
            return

        imported_count = 0
        for func_name, args_raw in matches:
            # Split arguments by comma, but ignore commas inside quotes
            args = [a.strip().strip('"') for a in re.split(r',(?=(?:[^"]*"[^"]*")*[^"]*$)', args_raw)]

            tool = self.find_tool_by_func(func_name)

            # We need at least x and y (index 0 and 1)
            if tool and len(args) >= 2:
                try:
                    x, y = int(args[0]), int(args[1])
                    el = DesignerElement(tool, x, y, self)

                    # Map the rest of the arguments to the property keys
                    # We skip x/y, so we start at args[2]
                    # Note: This assumes Rust args follow the order in definitions.xml
                    arg_keys = list(tool['args'].keys())
                    for i, val in enumerate(args[2:]):
                        if i < len(arg_keys):
                            el.properties[arg_keys[i]] = val

                    # Update visual size if width/height were imported
                    w = float(el.properties.get('width', el.rect().width()))
                    h = float(el.properties.get('height', el.rect().height()))
                    el.setRect(0, 0, w, h)

                    self.scene.addItem(el)
                    imported_count += 1
                except Exception as e:
                    print(f"Failed to import {func_name}: {e}")
                    continue

        QMessageBox.information(self, "Import", f"Successfully imported {imported_count} elements.")

if __name__ == "__main__":
    app = QApplication(sys.argv)
    ex = PixelDesignerApp("definitions.xml")
    ex.show()
    sys.exit(app.exec())