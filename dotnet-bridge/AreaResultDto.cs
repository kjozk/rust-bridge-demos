using MessagePack;
namespace DotnetBridge;

[MessagePackObject]
public sealed class AreaResultDto
{
    [Key("area")]
    public double Area { get; set; }
}
