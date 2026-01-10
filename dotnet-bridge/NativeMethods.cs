using System;
using System.Runtime.InteropServices;
namespace DotnetBridge;

internal static class NativeMethods
{
    [DllImport(
        "native/messagepack_bridge",
        EntryPoint = "calc_area_msgpack_ffi",
        CallingConvention = CallingConvention.Cdecl)]
    internal static extern int CalcAreaMsgpackFfi(
        byte[] input,
        nuint inputLen,
        out IntPtr outputPtr,
        out nuint outputLen);

    [DllImport(
        "native/messagepack_bridge",
        EntryPoint = "free_buffer",
        CallingConvention = CallingConvention.Cdecl)]
    internal static extern void FreeBuffer(
        IntPtr ptr,
        nuint len);
}
