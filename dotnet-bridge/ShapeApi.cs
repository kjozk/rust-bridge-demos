namespace DotnetBridge;
using MessagePack;
using System.Runtime.InteropServices;

public static class ShapeApi
{
    public static double CalculateRectangleArea(RectangleDto rect)
    {
        var input = MessagePackSerializer.Serialize(rect);

        IntPtr ptr = NativeMethods.calc_rectangle_area_msgpack(
            input,
            input.Length,
            out int outputLen);

        if (ptr == IntPtr.Zero)
            throw new InvalidOperationException("Native call failed");

        try
        {
            var output = new byte[outputLen];
            Marshal.Copy(ptr, output, 0, outputLen);

            var result = MessagePackSerializer.Deserialize<AreaResultDto>(output);
            return result.Area;
        }
        finally
        {
            NativeMethods.free_buffer(ptr, (nuint)outputLen);
        }
    }
}
