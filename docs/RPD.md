# edid-rs 设计文档（RPD.md, v0.2）

## 1. 项目目标

实现一个纯 Rust 的 EDID 解析与编辑库，具备：

* 从二进制解析 EDID（Base + Extensions）
* 类型安全访问字段
* 支持字段编辑（内存级）
* 可序列化回合法二进制
* 保证 round-trip（无修改时字节等价）

---

## 2. 非目标（Out of Scope）

* 不操作硬件（I2C / DDC / DRM）
* 不负责 EDID 注入或 override
* 不实现 GUI（仅库 + 可选 CLI）

---

## 3. 关键现实与设计约束（新增）

### 3.1 规范碎片化（核心问题）

EDID 实际由多套标准拼接而成：

* EDID 1.3 / 1.4（Base）
* E-EDID（扩展机制）
* CTA-861（HDMI / 音视频能力）
* HDMI Vendor Specific Block（部分封闭）
* DisplayID（新一代）

**结论：**

* 文档“基本都有”，但分散且部分需付费/会员
* 存在未完全公开/灰色字段
* 不同标准之间交叉引用严重

---

### 3.2 工程结论

本库不追求“完全覆盖所有规范”，而是：

> 构建一个“容错 + 可扩展 + 可回写”的解析器

---

## 4. EDID 基础结构

### 4.1 Base Block（128 bytes）

（略，保持 v0.1）

---

## 5. 核心设计原则（新增重点）

### 5.1 强类型 + Raw 共存

```rust
pub struct Edid {
    pub raw: Vec<u8>,          // 原始数据（必须保留）
    pub base: BaseBlock,
    pub extensions: Vec<ExtensionBlock>,
}
```

**原因：**

* 支持未知字段
* 保证 round-trip
* 调试与兼容性

---

### 5.2 容错解析

```rust
pub enum ExtensionBlock {
    Cta861(Cta861Extension),
    Unknown(Vec<u8>),
}
```

```rust
pub enum Descriptor {
    DetailedTiming(...),
    MonitorName(String),
    Unknown([u8; 18]),
}
```

---

### 5.3 读写分离

```rust
parse() -> struct
to_bytes() -> binary
```

---

### 5.4 Round-trip 保证（核心质量指标）

```text
parse → serialize == 原始数据（未修改时）
```

---

## 6. 模块结构

```text
edidkit/
├── base/
├── cta861/
├── displayid/
├── edid/
├── utils/
└── error.rs
```

公开 API 按协议导出：`edidkit::base`、`edidkit::cta861`、`edidkit::displayid`。

---

## 7. 解析架构

### 7.1 流程

1. 校验长度
2. 校验 Header
3. 解析 Base Block
4. 读取 extension count
5. 逐块解析 extension
6. checksum 校验

---

### 7.2 Extension 分层（关键）

```rust
pub enum ExtensionBlock {
    Cta861(Cta861Extension),
    Unknown(Vec<u8>),
}
```

---

### 7.3 CTA-861（阶段性支持）

```rust
pub struct Cta861Extension {
    pub data_blocks: Vec<DataBlock>,
}
```

```rust
pub enum DataBlock {
    Video(VideoBlock),
    Audio(AudioBlock),
    Vendor(VendorBlock),
    Unknown(Vec<u8>),
}
```

---

## 8. 编辑能力

### 8.1 API

```rust
impl Edid {
    pub fn set_product_code(&mut self, code: u16);
    pub fn set_monitor_name(&mut self, name: &str);
}
```

---

### 8.2 编辑策略

* 修改 struct，不直接改 raw
* serialize 时重新生成
* 未识别字段保持原样

---

## 9. 序列化

### 9.1 流程

1. 写 Base Block
2. 写 Descriptor
3. 写 Extension
4. 自动 checksum

---

### 9.2 Checksum

```rust
fn fix_checksum(block: &mut [u8]) {
    let sum: u8 = block[..127].iter().fold(0, |a, b| a.wrapping_add(*b));
    block[127] = (256 - sum as u16) as u8;
}
```

---

## 10. Bitfield 处理（新增）

建议统一抽象：

```rust
struct BitReader;
struct BitWriter;
```

避免散落位运算：

```rust
(x >> 5) & 0x07
```

---

## 11. 错误处理

```rust
pub enum EdidError {
    InvalidLength,
    InvalidHeader,
    InvalidChecksum,
    UnsupportedExtension,
    ParseError(String),
}
```

---

## 12. 渐进实现路线（关键）

### v0.1

* Base Block
* Descriptor
* serialize
* checksum
* round-trip

### v0.2

* CTA-861（Video / Audio）

### v0.3

* HDMI Vendor Block
* HDR Metadata

---

## 13. 测试策略（强化）

### 13.1 数据来源

* 实际设备 EDID
* Linux `/sys/class/drm`
* 公开样本库

### 13.2 测试类型

* 单元测试
* round-trip 测试（必须）
* 模糊测试（fuzz）

---

## 14. 性能设计

* 尽量使用 slice（减少拷贝）
* 延迟解析（Descriptor / CTA）
* 避免 clone

---

## 15. 风险与限制（新增）

### 15.1 HDMI Vendor Block 不完全公开

解决方案：

* 参考开源实现
* 保留 raw

---

### 15.2 规范差异

* 不同版本字段含义不同
* 需版本感知解析

---

### 15.3 未知字段

解决：

```rust
Unknown(Vec<u8>)
```

---

## 16. 工程关键能力（总结）

本库必须具备：

* ✅ 容错解析
* ✅ 强类型访问
* ✅ raw 保留
* ✅ round-trip
* ✅ 可扩展 extension

---

## 17. 示例

```rust
let edid = Edid::parse(&data)?;
println!("{}", edid.base.manufacturer_id);

let mut edid = edid;
edid.set_product_code(1234);

let new_bytes = edid.to_bytes();
```

---

## 18. 最终结论

EDID 实现难点不在“解析”，而在：

* 多标准拼接
* 部分规范封闭
* 历史兼容负担

本设计通过：

* 分层模型
* 容错机制
* raw 保留
* 渐进实现

实现一个**工程可用、可扩展、可维护的 EDID Rust 库**。
