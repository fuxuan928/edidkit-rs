# 🧠 一、设计定位（edidkit vs edid-rs）

## 👉 edid-rs（现状总结）

* 偏“解析 demo”
* 覆盖有限（主要 base block）
* 基本不可编辑 / 不可回写
* 类型设计较弱

---

## 👉 edidkit（目标）

> 一个**工程级 EDID 解析 + 编辑 + round-trip 安全库**

核心能力：

* ✅ 完整解析（渐进支持 CTA）
* ✅ 可编辑字段
* ✅ 可序列化
* ✅ 保留 raw（关键改进）
* ✅ round-trip 保证

---

# 🧩 二、总体架构（对标 + 改进）

## 🔁 继承 edid-rs 的优点

* 简单清晰的 parse 流程
* Rust 风格 API
* 轻量实现

---

## 🚀 核心改进点（重点）

### ❗ 改进 1：从“解析结果” → “双层模型”

## ✅ edidkit：

```rust
pub struct Edid {
    pub raw: Vec<u8>,              // ⭐ 新增（关键）
    pub base: BaseBlock,
    pub extensions: Vec<ExtensionBlock>,
}
```

### 👉 为什么比 edid-rs 好？

| 能力         | edid-rs | edidkit |
| ---------- | ------- | ------- |
| 保留未知字段     | ❌       | ✅       |
| round-trip | ❌       | ✅       |
| debug      | ❌       | ✅       |

---

# 🧱 三、模块设计（升级版）

```text
edidkit/
├── base/          # Base EDID + descriptor
├── cta861/        # CTA-861 types + parse + write
├── displayid/     # DisplayID types + parse + write
├── edid/          # 顶层文档聚合与调度
├── lib.rs
├── utils/
└── error.rs
```

公开 API：

* `edidkit::Edid`
* `edidkit::ExtensionBlock`
* `edidkit::base::*`
* `edidkit::cta861::*`
* `edidkit::displayid::*`


---

# 🧠 四、数据结构设计（关键升级）

## 1️⃣ BaseBlock（改进）

### edid-rs（问题）

* 字段不完整
* 有些直接用 u8 / u16 暴露

---

### edidkit（改进）

```rust
pub struct BaseBlock {
    pub manufacturer_id: ManufacturerId,
    pub product_code: u16,
    pub serial_number: u32,
    pub manufacture_date: ManufactureDate,
    pub version: EdidVersion,
    pub descriptors: Vec<Descriptor>,
}
```

---

## 2️⃣ 强类型封装（新增）

```rust
pub struct ManufacturerId(pub String);

pub struct ManufactureDate {
    pub week: u8,
    pub year: u16,
}

pub struct EdidVersion {
    pub major: u8,
    pub minor: u8,
}
```

👉 好处：

* 避免裸类型误用
* 提高 API 可读性

---

## 3️⃣ Descriptor（重大改进）

### edid-rs：

```rust
[u8; 18]
```

---

### edidkit：

```rust
pub enum Descriptor {
    DetailedTiming(DetailedTiming),
    MonitorName(String),
    MonitorSerial(String),
    RangeLimits(RangeLimits),
    Unknown([u8; 18]),   // ⭐ 保底
}
```

👉 核心优势：

* 可编辑
* 可扩展
* 不丢数据

---

# 🔍 五、解析层设计（借鉴 + 优化）

## edid-rs：

* 单层 parse
* 直接解析到字段

---

## edidkit：

### ✅ 分层解析

```text
binary → raw block → typed struct
```

---

### 示例：

```rust
fn parse_base_block(data: &[u8]) -> BaseBlock
```

---

### 优化点：

* 每个 block 独立 parser
* extension 独立模块
* descriptor 独立解析

👉 好处：

* 可维护
* 易测试

---

# ✏️ 六、编辑能力（edid-rs 没有的核心能力）

## edidkit API：

```rust
impl Edid {
    pub fn set_monitor_name(&mut self, name: &str);
    pub fn set_product_code(&mut self, code: u16);
}
```

---

## 内部策略（关键）

❌ 不直接改 raw
✅ 修改 struct → serialize 时统一生成

---

# 🔄 七、序列化（重大新增）

## edid-rs：❌ 没有

---

## edidkit：

```rust
impl Edid {
    pub fn to_bytes(&self) -> Vec<u8>;
}
```

---

## 关键设计：

### 1️⃣ 保留 unknown block

```rust
Unknown(Vec<u8>)
```

👉 保证：

> 不认识的数据不会丢

---

### 2️⃣ checksum 自动处理

---

### 3️⃣ round-trip 保证

```text
parse → to_bytes == 原始数据（未修改）
```

---

# 🔌 八、CTA-861 设计（架构级改进）

## edid-rs：基本没有

---

## edidkit（分层设计）

```rust
pub struct Cta861Extension {
    pub data_blocks: Vec<DataBlock>,
}
```

---

```rust
pub enum DataBlock {
    Video(VideoBlock),
    Audio(AudioBlock),
    Vendor(VendorBlock),
    Unknown(Vec<u8>),
}
```

---

## 👉 关键设计思想

> “解析你理解的，保留你不理解的”

---

# 🧠 九、Bitfield 抽象（重要优化）

## edid-rs：手写位运算

---

## edidkit：

```rust
struct BitReader<'a> { ... }
struct BitWriter<'a> { ... }
```

---

👉 好处：

* 减少 bug
* 可读性高
* 易复用

---

# 🧪 十、测试能力（工程级提升）

## 新增：

### 1️⃣ round-trip 测试（必须）

```text
input.bin → parse → serialize → == input.bin
```

---

### 2️⃣ fuzz 测试

👉 防止异常 EDID 崩溃

---

### 3️⃣ 样本库测试

* 不同显示器
* 不同厂商

---

# ⚠️ 十一、关键风险 & 对策

| 风险         | 解决              |
| ---------- | --------------- |
| HDMI 规范不公开 | 保留 raw          |
| 未知字段       | Unknown variant |
| 解析错误       | 精细 error        |
| 标准变化       | 插件式扩展           |

---

# 🚀 十二、与 edid-rs 的核心差异总结

| 维度         | edid-rs | edidkit |
| ---------- | ------- | ------- |
| 解析         | 基础      | 完整（渐进）  |
| 类型系统       | 弱       | 强类型     |
| 编辑         | ❌       | ✅       |
| 写回         | ❌       | ✅       |
| raw 保留     | ❌       | ✅       |
| round-trip | ❌       | ✅       |
| CTA-861    | ❌       | ✅（设计支持） |

---

# 🧠 最关键的一句话

👉 edid-rs 是：

> “能读 EDID”

👉 edidkit 是：

> **“能安全修改并重新生成 EDID 的工程库”**
