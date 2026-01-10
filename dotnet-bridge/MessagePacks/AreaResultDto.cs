using MessagePack;
namespace DotnetBridge.MessagePacks;

[MessagePackObject]
public sealed class AreaResultDto
{
    [Key("area")]
    public double Area { get; set; }
}
