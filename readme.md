# tauri-plugin-penetrable</h1>


##### Using the win32api to achieve click-through of the tauri main window
##### 利用win32api实现tauri主窗口点击穿透
##### win32api を利用して、tauri メインウィンドウのクリックスルーを実現する

- ### Installation 
 
```bash
cargo add tauri-plugin-penetrable
```



```rust
use tauri_plugin_penetrable::Penetrable;

fn main() {
  tauri::Builder::default().plugin(
      Penetrable::init(),
  );
}
```
