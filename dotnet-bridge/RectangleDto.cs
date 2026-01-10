using MessagePack;

namespace DotnetBridge;

[MessagePackObject]
public sealed class RectangleDto
{
    [Key("width")]
    public double Width { get; set; }

    [Key("height")]
    public double Height { get; set; }
}
