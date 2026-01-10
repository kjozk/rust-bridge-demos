import py_bridge

rect = {"width": 3.0, "height": 4.0}
area = py_bridge.calc_rectangle_area(rect)
print(f"Area: {area}")
