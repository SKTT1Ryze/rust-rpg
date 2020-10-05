# **Rust RPG**

**A RPG Game Written with Rust.**  

## Dependencies
piston = "0.52.0"  
piston2d-graphics = "0.37.0"  
pistoncore-glutin_window = "0.66.0"  
piston2d-opengl_graphics = "0.74.0"  
log = "0.4.11"  


## Run
```Shell
cd game
cargo run
```
## Control
+ Up, Down, Left and Right
+ Type 1~9 to choose which game object to control
+ Much to do

## **Implement!**

+ 定义一个 `Manager` 的 trait 来实现对物体管理器的抽象，目前我定义了一个 `CubesManager` 的结构体并为其实现了 `Manager` trait
+ 实现了通过 `CubesManager` 对场景中的物体进行管理，具体上是通过数字键选择当前物体，然后通过方向键使选中的物体移动
+ 为 `CubesManager` 实现 `EventHandler` trait，使其可以响应键盘事件
+ 定义了一个 `MouseDeletor`，为其实现 `EventHandler` trait，使其可以捕获鼠标输入。不过可能没什么用，因为我打算将这个游戏打造成全键盘操作
+ 为 `CubePlay` 实现 `fmt::Debug` trait

## **Unimplement!**
So much  

## **I hope to finish this project in 2 mounts**