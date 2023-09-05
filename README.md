<div align="center">

![][banner]

</div>

## 简介

使用 Rust + Rodio 实现的音频播放器 使用 JNI 调用

## 开发

```shell
git clone https://github.com/RealHeart/ZMusic-Player.git zmusic-player
cd zmusic-player
cargo build
```

## 测试

测试代码: 

```java
package me.zhenxin.zmusic.music; // 包名不能更改 如需更改请更改 src/lib.rs 内的函数名称

import java.io.File;

public class JniPlayer {

  public JniPlayer() {
    System.loadLibrary("libzmusic");
  }

  private native void init();

  private native void load(String url);

  private native void play();

  private native void pause();

  private native void resume();

  private native void stop();

  private native float getVolume();

  private native void setVolume(float volume);

  private native int getStatus();

  public static void main(String[] args) {
    var url = ""; // 音频链接 支持: mp3, flac, wav, ogg, m4a, mp4, aac等
    System.out.println("running...");
    JniPlayer player = new JniPlayer();
    player.init();
    System.out.println("initialized.");
    System.out.println("loading " + url);
    player.load(url);
    player.setVolume(0.1f);
    System.out.println("volume: " + player.getVolume());
    System.out.println("playing...");
    player.play();
    while (true) {
      try {
        Thread.sleep(1000);
        System.out.println("status: " + statusToString(player.getStatus()));
      } catch (Exception e) {
        e.printStackTrace();
      }
    }
  }

  public static String statusToString(int status) {
    switch (status) {
      case 0:
        return "STOPPED";
      case 1:
        return "PLAYING";
      case 2:
        return "PAUSED";
      default:
        return "UNKNOWN";
    }
  }
}

```

运行测试:

```shell
java -Djava.library.path=target/debug JniPlayer.java # 调试
java -Djava.library.path=target/release JniPlayer.java # 生产
```

## 开源协议

本项目使用 [GPL-3.0](LICENSE) 协议开放源代码

```text
ZMusic Player
Copyright (C) 2023 ZhenXin
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
```

[banner]: https://socialify.git.ci/RealHeart/ZMusic-Player/image?description=1&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Auto
