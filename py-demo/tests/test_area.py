import unittest
import py_bridge

class TestRectangleArea(unittest.TestCase):

    def test_area_positive(self):
        rect = py_bridge.PyRectangle(3.0, 4.0)
        area = py_bridge.calc_rectangle_area(rect)
        self.assertEqual(area, 12.0)

    def test_area_zero(self):
        rect = py_bridge.PyRectangle(0.0, 10.0)
        area = py_bridge.calc_rectangle_area(rect)
        self.assertEqual(area, 0.0)

    def test_area_float(self):
        rect = py_bridge.PyRectangle(2.5, 4.0)
        area = py_bridge.calc_rectangle_area(rect)
        self.assertAlmostEqual(area, 10.0)

if __name__ == "__main__":
    unittest.main()
