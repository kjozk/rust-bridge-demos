using System;
using System.Runtime.InteropServices;
namespace DotnetBridge;

internal static class NativeMethods
{
    private const string DllName = "messagepack_bridge";

    [DllImport("messagepack_bridge", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr calc_rectangle_area_msgpack(
        byte[] input,
        int inputLen,
        out int outputLen);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void free_buffer(IntPtr ptr, UIntPtr len);
}
