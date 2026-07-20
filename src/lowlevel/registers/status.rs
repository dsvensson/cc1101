// status registers: single-byte, read-only, must use burst
bitfields!(status, read = Burst, write = Reject, {
    #[doc = "Chip part number"]
    PARTNUM @ 0x30 = 0b0000_0000 {
        #[doc = "Chip part number"]
        partnum @ 0..8,
    }
    #[doc = "Chip version number"]
    VERSION @ 0x31 = 0b0001_0100 {
        #[doc = "Chip version number"]
        version @ 0..8,
    }
    #[doc = "Frequency Offset Estimate from Demodulator"]
    FREQEST @ 0x32 = 0b0000_0000 {
        #[doc = "The estimated frequency offset (2's complement) of the carrier"]
        freqoff_est @ 0..8,
    }
    #[doc = "Demodulator Estimate for Link Quality"]
    LQI @ 0x33 = 0b0000_0000 {
        #[doc = "The last CRC comparison matched."]
        crc_ok @ 7,
        #[doc = "The Link Quality Indicator estimates how easily a received signal can be demodulated"]
        lqi @ 0..7,
    }
    #[doc = "Received Signal Strength Indication"]
    RSSI @ 0x34 = 0b0000_0000 {
        #[doc = "Received signal strength indicator"]
        rssi @ 0..8,
    }
    #[doc = "Main Radio Control State Machine State"]
    MARCSTATE @ 0x35 = 0b0000_0000 {
        #[doc = "Main Radio Control FSM State"]
        marc_state @ 0..5,
    }
    #[doc = "High Byte of WOR Time"]
    WORTIME1 @ 0x36 = 0b0000_0000 {
        #[doc = "High byte of timer value in WOR module"]
        time @ 0..8,
    }
    #[doc = "Low Byte of WOR Time"]
    WORTIME0 @ 0x37 = 0b0000_0000 {
        #[doc = "Low byte of timer value in WOR module"]
        time @ 0..8,
    }
    #[doc = "Current GDOx Status and Packet Status"]
    PKTSTATUS @ 0x38 = 0b0000_0000 {
        #[doc = "The last CRC comparison matched"]
        crc_ok @ 7,
        #[doc = "Carrier sense"]
        cs @ 6,
        #[doc = "Preamble Quality reached"]
        pqt_reached @ 5,
        #[doc = "Channel is clear"]
        cca @ 4,
        #[doc = "Start of Frame Delimiter"]
        sfd @ 3,
        #[doc = "Current GDO2 value"]
        gdo2 @ 2,
        #[doc = "Current GDO0 value"]
        gdo0 @ 0,
    }
    #[doc = "Current Setting from PLL Calibration Module"]
    VCO_VC_DAC @ 0x39 = 0b0000_0000 {
        #[doc = "Status register for test only"]
        vco_vc_dac @ 0..8,
    }
    #[doc = "Underflow and Number of Bytes"]
    TXBYTES @ 0x3A = 0b0000_0000 {
        #[doc = "TX FIFO underflow"]
        txfifo_underflow @ 7,
        #[doc = "Number of bytes in TX FIFO"]
        num_txbytes @ 0..7,
    }
    #[doc = "Overflow and Number of Bytes"]
    RXBYTES @ 0x3B = 0b0000_0000 {
        #[doc = "RX FIFO overflow"]
        rxfifo_overflow @ 7,
        #[doc = "Number of bytes in RX FIFO"]
        num_rxbytes @ 0..7,
    }
    #[doc = "Last RC Oscillator Calibration Result"]
    RCCTRL1_STATUS @ 0x3C = 0b0000_0000 {
        #[doc = "Contains the value from the last run of the RC oscillator calibration routine"]
        rcctrl1_status @ 0..7,
    }
    #[doc = "Last RC Oscillator Calibration Result"]
    RCCTRL0_STATUS @ 0x3D = 0b0000_0000 {
        #[doc = "Contains the value from the last run of the RC oscillator calibration routine"]
        rcctrl0_status @ 0..7,
    }
});
