using System.ComponentModel;
using System.Runtime.CompilerServices;
using DotnetBridge;
using DotnetBridge.MessagePacks;

namespace DotnetDemo.ViewModels;

public class MainViewModel : INotifyPropertyChanged
{
    private double _width = 10;
    private double _height = 20;
    private double _area;

    public double Width
    {
        get => _width;
        set { _width = value; OnPropertyChanged(); }
    }

    public double Height
    {
        get => _height;
        set { _height = value; OnPropertyChanged(); }
    }

    public double Area
    {
        get => _area;
        private set { _area = value; OnPropertyChanged(); }
    }

    public void Calculate()
    {
        var rect = new RectangleDto
        {
            Width = Width,
            Height = Height
        };

        Area = ShapeApi.CalculateRectangleArea(rect);
    }

    public event PropertyChangedEventHandler? PropertyChanged;
    private void OnPropertyChanged([CallerMemberName] string? name = null)
        => PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(name));
}
