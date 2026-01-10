using System;
using System.Runtime.InteropServices;
namespace dotnet_bridge;

public static class MessagePackBridge
{
    public static byte[] Call(byte[] request)
    {
        if (request == null)
            throw new ArgumentNullException(nameof(request));

        int rc = NativeMethods.calc_area_msgpack(
            request,
            (UIntPtr)request.Length,
            out var ptr,
            out var len
        );

        if (rc != 0)
            throw new InvalidOperationException($"Native error code: {rc}");

        try
        {
            byte[] result = new byte[(int)len];
            Marshal.Copy(ptr, result, 0, result.Length);
            return result;
        }
        finally
        {
            NativeMethods.free_buffer(ptr, len);
        }
    }
}
