```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chip {
    Sn76489,
    Ym2612,
}

// チップ書き込みコマンドのトレイト
pub trait ChipCommand: std::fmt::Debug {
    fn chip(&self) -> Chip;
    fn to_vgm_bytes(&self) -> Vec<u8>;
}

// VGM全体のコマンド
#[derive(Debug)]
enum VgmCommand {
    Write(usize, Box<dyn ChipCommand>),
    Wait(u16),
    End,
}

// ビルダー本体
pub struct VgmBuilder {
    commands: Vec<VgmCommand>,
    total_samples: u32,
    loop_point: Option<usize>,  // ループ開始のコマンドインデックス
}

impl VgmBuilder {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            total_samples: 0,
            loop_point: None,
        }
    }

    // チップ書き込み追加（ジェネリクスで楽ちん）
    pub fn write<C>(&mut self, port: u32, cmd: C)
    where
        C: ChipCommand
    {
        self.commands.push(VgmCommand::Write(port, Box::new(cmd)));
    }

    pub fn wait(&mut self, samples: u16) {
        if samples > 0 {
            self.total_samples += samples as u32;
            self.commands.push(VgmCommand::Wait(samples));
        }
    }

    pub fn set_loop(&mut self) {
        self.loop_point = Some(self.commands.len());
    }

    pub fn end(&mut self) {
        self.commands.push(VgmCommand::End);
    }

    // 最終出力
    pub fn build<W: Write>(self, writer: &mut W) -> Result<()> {
        let mut data = Vec::new();

        for cmd in self.commands {
            match cmd {
                VgmCommand::Write(c) => data.extend(c.to_vgm_bytes()),
                VgmCommand::Wait(s) => data.extend(wait_to_bytes(s)),
                VgmCommand::End => data.push(0x66),
            }
        }

        // ここにヘッダー構築（簡易版、v1.71準拠）
        build_header_and_write(writer, &data, self.total_samples, self.loop_point, data.len() as u32)
    }
}

// ======== チップコマンド実装 ========

// SN76489 (PSG)
#[derive(Debug)]
pub struct PsgWrite {
    pub data: u8,
}

impl ChipCommand for PsgWrite {
    fn chip(&self) -> Chip { Chip::Sn76489 }
    fn to_vgm_bytes(&self) -> Vec<u8> {
        vec![0x50, self.data]
    }
}

// YM2612
#[derive(Debug)]
pub struct Ym2612Write {
    pub port: u8,     // 0 or 1
    pub register: u8,
    pub data: u8,
}

impl ChipCommand for Ym2612Write {
    fn chip(&self) -> Chip { Chip::Ym2612 }
    fn to_vgm_bytes(&self) -> Vec<u8> {
        let base = if self.port == 0 { 0x52 } else { 0x53 };
        vec![base, self.register, self.data]
    }
}
```