use crate::{
    base::{BaseBlock, Descriptor},
    cta861::Cta861Extension,
    displayid::DisplayIdExtension,
    error::EdidError,
    utils::{encode_manufacturer_id, validate_descriptor_text},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionBlock {
    Cta861(Cta861Extension),
    DisplayId(DisplayIdExtension),
    Unknown(Vec<u8>),
}

impl ExtensionBlock {
    pub fn raw_bytes(&self) -> &[u8] {
        match self {
            Self::Cta861(cta) => &cta.raw_block,
            Self::DisplayId(display_id) => &display_id.raw_block,
            Self::Unknown(bytes) => bytes,
        }
    }
}

/// A parsed EDID document with a typed base block and parsed extensions.
#[derive(Debug, Clone, PartialEq)]
pub struct Edid {
    pub(crate) raw: Vec<u8>,
    /// The parsed 128-byte base EDID block.
    pub base: BaseBlock,
    /// Parsed extension blocks in on-wire order.
    pub extensions: Vec<ExtensionBlock>,
}

/// A product-oriented view of the editable EDID identity fields.
///
/// This type is intended for serde-like workflows:
/// read a snapshot with [`Edid::product`], modify fields, then write it back
/// with [`Edid::set_product`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    /// Three-letter uppercase manufacturer ID.
    pub manufacturer_id: String,
    /// Product code stored in the base block.
    pub product_code: u16,
    /// 32-bit serial number stored in the base block.
    pub serial_number: u32,
    /// Optional monitor name descriptor.
    pub monitor_name: Option<String>,
    /// Optional monitor serial descriptor.
    pub monitor_serial: Option<String>,
}

impl Edid {
    /// Parses a complete EDID byte slice into a typed model.
    pub fn parse(data: &[u8]) -> Result<Self, EdidError> {
        super::parse::parse_edid(data)
    }

    /// Returns the original byte slice used when this EDID was parsed.
    ///
    /// This snapshot is not updated after mutations. Use [`Edid::to_bytes`]
    /// to serialize the current in-memory state.
    pub fn original_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Serializes the current in-memory EDID state back to bytes.
    ///
    /// Unlike earlier versions of this crate, serialization now returns an
    /// error instead of silently falling back to stale raw bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, EdidError> {
        super::write::write_edid(self)
    }

    /// Updates only the base block product code.
    pub fn set_product_code(&mut self, code: u16) {
        self.apply_product_identity(None, code, None);
    }

    /// Returns a product-oriented snapshot of the editable identity fields.
    pub fn product(&self) -> Product {
        Product {
            manufacturer_id: self.base.manufacturer_id.0.clone(),
            product_code: self.base.product_code,
            serial_number: self.base.serial_number,
            monitor_name: descriptor_text(&self.base.descriptors, DescriptorKind::MonitorName),
            monitor_serial: descriptor_text(&self.base.descriptors, DescriptorKind::MonitorSerial),
        }
    }

    /// Returns the current monitor name descriptor, if present.
    pub fn monitor_name(&self) -> Option<&str> {
        descriptor_text_ref(&self.base.descriptors, DescriptorKind::MonitorName)
    }

    /// Returns the current monitor serial descriptor, if present.
    pub fn monitor_serial(&self) -> Option<&str> {
        descriptor_text_ref(&self.base.descriptors, DescriptorKind::MonitorSerial)
    }

    /// Applies a product-oriented snapshot back onto the EDID model.
    ///
    /// Base block identity fields are overwritten directly. Optional text
    /// descriptors are updated in place, inserted into an unused descriptor
    /// slot, or removed when set to `None`.
    pub fn set_product(&mut self, product: &Product) -> Result<(), EdidError> {
        validate_manufacturer_id(product.manufacturer_id.as_str())?;

        self.apply_product_identity(
            Some(product.manufacturer_id.as_str()),
            product.product_code,
            Some(product.serial_number),
        );

        set_text_descriptor(
            &mut self.base.descriptors,
            DescriptorKind::MonitorName,
            product.monitor_name.as_deref(),
        )?;
        set_text_descriptor(
            &mut self.base.descriptors,
            DescriptorKind::MonitorSerial,
            product.monitor_serial.as_deref(),
        )?;

        Ok(())
    }

    /// Sets the monitor name descriptor.
    ///
    /// If the descriptor already exists it is updated in place. Otherwise an
    /// unused base descriptor slot is reused.
    pub fn set_monitor_name(&mut self, name: &str) -> Result<(), EdidError> {
        set_text_descriptor(
            &mut self.base.descriptors,
            DescriptorKind::MonitorName,
            Some(name),
        )
    }

    /// Sets the monitor serial descriptor.
    ///
    /// If the descriptor already exists it is updated in place. Otherwise an
    /// unused base descriptor slot is reused.
    pub fn set_monitor_serial(&mut self, serial: &str) -> Result<(), EdidError> {
        set_text_descriptor(
            &mut self.base.descriptors,
            DescriptorKind::MonitorSerial,
            Some(serial),
        )
    }

    fn apply_product_identity(
        &mut self,
        manufacturer_id: Option<&str>,
        product_code: u16,
        serial_number: Option<u32>,
    ) {
        if let Some(manufacturer_id) = manufacturer_id {
            self.base.manufacturer_id.0 = manufacturer_id.to_owned();
        }

        self.base.product_code = product_code;

        if let Some(serial_number) = serial_number {
            self.base.serial_number = serial_number;
        }
    }
}

#[derive(Clone, Copy)]
enum DescriptorKind {
    MonitorName,
    MonitorSerial,
}

fn descriptor_text(descriptors: &[Descriptor], kind: DescriptorKind) -> Option<String> {
    descriptor_text_ref(descriptors, kind).map(str::to_owned)
}

fn descriptor_text_ref<'a>(descriptors: &'a [Descriptor], kind: DescriptorKind) -> Option<&'a str> {
    descriptors
        .iter()
        .find_map(|descriptor| match (kind, descriptor) {
            (DescriptorKind::MonitorName, Descriptor::MonitorName(text)) => Some(text.as_str()),
            (DescriptorKind::MonitorSerial, Descriptor::MonitorSerial(text)) => Some(text.as_str()),
            _ => None,
        })
}

fn set_text_descriptor(
    descriptors: &mut [Descriptor; 4],
    kind: DescriptorKind,
    value: Option<&str>,
) -> Result<(), EdidError> {
    let index = descriptors
        .iter()
        .position(|descriptor| match (kind, descriptor) {
            (DescriptorKind::MonitorName, Descriptor::MonitorName(_)) => true,
            (DescriptorKind::MonitorSerial, Descriptor::MonitorSerial(_)) => true,
            _ => false,
        });

    match value {
        Some(text) => {
            validate_descriptor_text(text)?;
            if let Some(index) = index {
                descriptors[index] = make_text_descriptor(kind, text);
                return Ok(());
            }
            if let Some(index) = descriptors
                .iter()
                .position(|descriptor| matches!(descriptor, Descriptor::Unused))
            {
                descriptors[index] = make_text_descriptor(kind, text);
                return Ok(());
            }
            Err(EdidError::ValidationError(format!(
                "{} descriptor cannot be added because all four descriptor slots are in use",
                descriptor_label(kind)
            )))
        }
        None => {
            if let Some(index) = index {
                descriptors[index] = Descriptor::Unused;
            }
            Ok(())
        }
    }
}

fn make_text_descriptor(kind: DescriptorKind, text: &str) -> Descriptor {
    match kind {
        DescriptorKind::MonitorName => Descriptor::MonitorName(text.to_owned()),
        DescriptorKind::MonitorSerial => Descriptor::MonitorSerial(text.to_owned()),
    }
}

fn descriptor_label(kind: DescriptorKind) -> &'static str {
    match kind {
        DescriptorKind::MonitorName => "monitor name",
        DescriptorKind::MonitorSerial => "monitor serial",
    }
}

fn validate_manufacturer_id(id: &str) -> Result<(), EdidError> {
    encode_manufacturer_id(id).map(|_| ())
}
