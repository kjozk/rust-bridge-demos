namespace DotnetBridge;
using MessagePack;
using System.Runtime.InteropServices;

public static class ShapeApi
{
    public static double CalculateRectangleArea(RectangleDto rect)
    {
        var input = MessagePackSerializer.Serialize(rect);

        int rc = NativeMethods.CalcAreaMsgpackFfi(
            input,
            (nuint)input.Length,
            out IntPtr outputPtr,
            out nuint outputLen);

        if (rc != 0 || outputPtr == IntPtr.Zero)
            throw new InvalidOperationException($"Native error rc={rc}");

        try
        {
            var output = new byte[(int)outputLen];
            Marshal.Copy(outputPtr, output, 0, (int)outputLen);

            var result = MessagePackSerializer.Deserialize<AreaResultDto>(output);
            return result.Area;
        }        finally
        {
            NativeMethods.FreeBuffer(outputPtr, outputLen);
        }
    }
}
