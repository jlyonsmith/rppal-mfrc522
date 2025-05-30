//! MFRC522 register definitions

/// List of all registers for the MFRC522
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    /// Starts and stops command execution.
    CommandReg = 0x01,
    /// Control bits to enable and disable the passing of interrupt requests.
    ComlEnReg = 0x02,
    /// Control bits to enable and disable the passing of interrupt requests.
    DivlEnReg = 0x03,
    /// Interrupt request bits.
    ComIrqReg = 0x04,
    /// Interrupt request bits.
    DivIrqReg = 0x05,
    /// Error bit register showing the error status of the last command executed.
    ErrorReg = 0x06,
    /// Contains status bits of the CRC, interrupt and FIFO buffer.
    Status1Reg = 0x07,
    /// Contains status bits of the receiver, transmitter and data mode detector.
    Status2Reg = 0x08,
    /// Input and output of 64 byte FIFO buffer.
    FIFODataReg = 0x09,
    /// Indicates the number of bytes stored in the FIFO.
    FIFOLevelReg = 0x0A,
    /// Defines the level for FIFO under- and overflow warning.
    WaterLevelReg = 0x0B,
    /// Miscellaneous control bits.
    ControlReg = 0x0C,
    /// Adjustments for bit-oriented frames.
    BitFramingReg = 0x0D,
    /// Defines the first bit-collision detected on the RF interface.
    CollReg = 0x0E,
    /// Defines general mode settings for transmitting and receiving.
    ModeReg = 0x11,
    /// Defines the data rate during transmission.
    TxModeReg = 0x12,
    /// Defines the data rate during reception.
    RxModeReg = 0x13,
    /// Controls the logical behavior of the antenna driver pins TX1 and TX2.
    TxControlReg = 0x14,
    /// Controls transmit modulation settings.
    TxASKReg = 0x15,
    /// Selects the internal sources for the analog module.
    TxSelReg = 0x16,
    /// Selects internal receiver settings.
    RxSelReg = 0x17,
    /// Selects thresholds for the bit decoder.
    RxThresholdReg = 0x18,
    /// Defines demodulator settings.
    DemodReg = 0x19,
    /// Controls some MIFARE communication transmit parameters.
    MfTxReg = 0x1C,
    /// Controls some MIFARE communication receive parameters.
    MfRxReg = 0x1D,
    /// Selects the speed of the serial UART interface.
    SerialSpeedReg = 0x1F,
    /// CRC calculation highest 8 bits.
    CRCResultRegHigh = 0x21,
    /// CRC calculation lowest 8 bits.
    CRCResultRegLow = 0x22,
    /// Sets the modulation width.
    ModWidthReg = 0x24,
    /// Configures the receiver gain.
    RFCfgReg = 0x26,
    /// Defines the conductance of the antenna driver pins TX1 and TX2 for the n-driver
    /// when the driver is switched on.
    GsNReg = 0x27,
    /// Defines the conductance of the p-driver output during periods of no modulation.
    CWGsPReg = 0x28,
    /// Defines the conductance of the p-driver output during modulation.
    ModGsPReg = 0x29,
    /// Timer settings + prescaler highest 4 bits.
    TModeReg = 0x2A,
    /// Timer prescaler lowest 8 bits.
    TPrescalerReg = 0x2B,
    /// Timer reload value highest 8 bits.
    TReloadRegHigh = 0x2C,
    /// Timer reload value lowest 8 bits.
    TReloadRegLow = 0x2D,
    /// Timer value highest 8 bits.
    TCounterValRegHigh = 0x2E,
    /// Timer value lower 8 bits.
    TCounterValRegLow = 0x2F,
    /// General test signal configuration.
    TestSel1Reg = 0x31,
    /// General test signal configuration and PRBS control.
    TestSel2Reg = 0x32,
    /// Enables the test bus pin output driver.
    TestPinEnReg = 0x33,
    /// Defines the HIGH and LOW values for the test port D1 to D7 when it is used as I/O.
    TestPinValueReg = 0x34,
    /// Shows the status of the internal test bus.
    TestBusReg = 0x35,
    /// Controls the digital self-test.
    AutoTestReg = 0x36,
    /// Shows the MFRC522 software version.
    VersionReg = 0x37,
    /// Determines the analog output test signal at, and status of, pins AUX1 and AUX2.
    AnalogTestReg = 0x38,
    /// Defines the test value for TestDAC1.
    TestDAC1Reg = 0x39,
    /// Defines the test value for TestDAC2.
    TestDAC2Reg = 0x3A,
    /// Shows the values of ADC I and Q channels.
    TestADCReg = 0x3B,
}

impl From<Register> for u8 {
    #[inline(always)]
    fn from(variant: Register) -> Self {
        variant as _
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
/// The receiver's signal voltage gain factor
///
/// See the *RFCfgReg register* section in the datasheet
pub enum RxGain {
    /// 18 dB, minimum
    // 0x20 is also an alias for it
    DB18 = 0x00,
    /// 23 dB
    // 0x30 is also an alias for it
    DB23 = 0x10,
    /// 33 dB, average
    DB33 = 0x40,
    /// 38 dB
    DB38 = 0x50,
    /// 43 dB
    DB43 = 0x60,
    /// 48 dB, maximum
    DB48 = 0x70,
}

impl From<RxGain> for u8 {
    #[inline(always)]
    fn from(variant: RxGain) -> Self {
        variant as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
/// List of different commands for the MFRC522
/// with their *code* as value.
pub enum Command {
    /// No action, cancels current command.
    Idle = 0b0000,
    /// Store 25 bytes into the internal buffer.
    Mem = 0b0001,
    /// Generate a 10-byte random ID.
    GenerateRandomId = 0b0010,
    /// Activate the CRC coprocessor.
    CalcCRC = 0b0011,
    /// Transmit data from the FIFO buffer (to the antenna).
    Transmit = 0b0100,
    /// Modify other *CommandReg* bits without affecting the command.
    NoCmdChange = 0b0111,
    /// Activate the receiver circuit.
    Receive = 0b1000,
    /// Transmit data from the FIFO buffer and activate the receiver after transmission.
    Transceive = 0b1100,
    /// Perform MIFARE standard authentication as a reader.
    MFAuthent = 0b1110,
    /// Reset the MFRC522.
    SoftReset = 0b1111,
}

impl From<Command> for u8 {
    #[inline(always)]
    fn from(variant: Command) -> Self {
        variant as _
    }
}
