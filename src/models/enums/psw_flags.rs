pub enum PSWFlags {
    Negative = 0b1000_0000,             // N
    Overflow = 0b0100_0000,             // V
    DirectPage = 0b0010_0000,           // P
    Break = 0b0001_0000,                // B
    HalfCarry = 0b0000_1000,            // H
    InterruptEnabled = 0b0000_0100,     // I
    Zero = 0b0000_0010,                 // Z
    Carry = 0b0000_0001,                // C
}
