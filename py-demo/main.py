import py_bridge

rect = py_bridge.PyRectangle(3.0, 4.0)
area = py_bridge.calc_rectangle_area(rect)
print(f"Area: {area}")
