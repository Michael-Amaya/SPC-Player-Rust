#[allow(unused)]
#[derive(Debug, Default, Clone, Copy)]
pub enum AddressingModes {
    #[default]
    DirectPage,
    XIndexedDirectPage,
    YIndexedDirectPage,
    Indirect,
    IndirectAutoIncrement,
    DirectPageToDirectPage,
    IndirectPageToIndirectPage,
    ImmediateDataToDirectPage,
    DirectPageBit,
    DirectPageBitRelative,
    AbsoluteBooleanBit,
    Absolute,
    AbsoluteXIndexedIndirect,
    XIndexedAbsolute,
    YIndexedAbsolute,
    XIndexedIndirect,
    IndirectYIndexed,
    Relative,
    Immediate,
    Accumulator,
    Implied,
}