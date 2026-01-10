using System;
using System.Runtime.InteropServices;
namespace dotnet_bridge;

internal static class NativeMethods
{
    private const string DllName = "messagepack_bridge";

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern int calc_area_msgpack(
        byte[] input,
        UIntPtr input_len,
        out IntPtr output_ptr,
        out UIntPtr output_len
    );

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    internal static extern void free_buffer(IntPtr ptr, UIntPtr len);
}
