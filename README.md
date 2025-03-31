# egui-scale

`egui-scale` is a Rust library that provides a trait for applying scale transformations to various types used in the [egui](https://github.com/emilk/egui) GUI framework. This library is designed to make it easy to scale UI elements consistently across your application.

## Features

- **Generic Scale Trait**: The `EguiScale` trait can be implemented for any type, allowing flexible scaling.
- **Built-in Implementations**: Predefined `EguiScale` implementations for common `egui` types, such as:
    - `Vec2`
    - `Margin`
    - `Stroke`
    - `Visuals`
    - `Style`
    - And many more!
- **Customizable**: Extend the functionality by implementing the `EguiScale` trait for your own types.

## Example Usage

```rust
use egui_scale::EguiScale;

fn show_large_labels(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.style_mut().scale(2.0);
        ui.label("This is a large label");
        ui.label("This is another large label");
    });
}
```

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
egui-scale = "0.1.0"
```

## Why Use `egui-scale`?

When building scalable and responsive UIs with `egui`, you often need to adjust sizes, margins, and other visual properties dynamically. `egui-scale` simplifies this process by providing a unified interface for scaling these properties.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/your-repo/egui-scale).

## License

This project is licensed under the [MIT License](LICENSE).

---

Happy coding with `egui-scale`! 🚀  