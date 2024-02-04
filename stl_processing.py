import sys 
from PyQt5.QtWidgets import QApplication, QMainWindow, QVBoxLayout, QWidget, QOpenGLWidget, QFileDialog, QPushButton
from OpenGL.GL import *
from OpenGL.GLUT import *
from OpenGL.GLU import *
from stl import mesh
import json

class STLViewer(QOpenGLWidget):
    def __init__ (self, parent = None):
        super().__init__(parent)
        self.stl_mesh = None

    def initializeGL(self):
        glClearColor(0.0, 0.0, 0.0, 1.0)
        glEnable(GL_DEPTH_TEST)

    def paintGL(self):
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        glLoadIdentity()
        glTranslatef(0.0, 0.0, -200.0)

        if self.stl_mesh:
            self.stl_mesh.draw()

    def resizeGL(self, width, height):
        glViewport(0, 0, width, height)
        glMatrixMode(GL_PROJECTION)
        glLoadIdentity()
        gluPerspective(45, width / height, 0.1, 1000.0)
        glMatrixMode(GL_MODELVIEW)

    def load_stl_file(self, file_path):
        try: 
            self.stl_mesh = mesh.Mesh.from_file(file_path)
            self.update()
        except Exception as e: 
            print("Erm. Error loading STL file:", e)

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("STL Viewer")
        self.setGeometry(100, 100, 800, 600)

        self.stl_viewer = STLViewer()
        self.seteCentralWidget(self.stl_viewer)

        open_button = QPushButton("Open STL File", self)
        open_button.clicked.connect(self.open_stl_file)

        layout = QVBoxLayout()
        layout.addWidget(open_button)
        layout.addWidget(self.stl_viewer)

        central_widget = QWidget()
        central_widget.setLayout(layout)
        self.setsCentralWidget(central_widget)

    def open_stl_file(self):
        file_path, _ = QFileDialog.getOpenFileName(self, "Open STL File", "", "STL files (*.stl)")
        if file_path: 
            self.stl_viewer.load_stl_file(file_path)

if __name__ == "__main__":
    app = QApplication(sys.argv)

    window = MainWindow()
    window.show()

    sys.exit(app.exec())

