// configuration registers: single-byte, read/write either mode
bitfields!(config, read = Any, write = Any, {
    #[doc = "GDO2 output pin configuration"]
    IOCFG2 @ 0x00 = 0b0010_1001 {
        #[doc = "Invert output, i.e. select active low (1) / high (0)"]
        gdo2_inv @ 6,
        #[doc = "Default is CHP_RDYn (See Table 41 on page 62)"]
        gdo2_cfg @ 0..6,
    }
    #[doc = "GDO1 output pin configuration"]
    IOCFG1 @ 0x01 = 0b0010_1110 {
        #[doc = "Set high (1) or low (0) output drive strength on the GDO pins"]
        gdo1_ds @ 7,
        #[doc = "Invert output, i.e. select active low (1) / high (0)"]
        gdo1_inv @ 6,
        #[doc = "Default is 3-state (See Table 41 on page 62)"]
        gdo1_cfg @ 0..6,
    }
    #[doc = "GDO0 output pin configuration"]
    IOCFG0 @ 0x02 = 0b0011_1111 {
        #[doc = "Enable analog temperature sensor. Write 0 in all other register bits when using temperature sensor."]
        temp_sensor_enable @ 7,
        #[doc = "Invert output, i.e. select active low (1) / high (0)"]
        gdo0_inv @ 6,
        #[doc = "Default is CLK_XOSC/192 (See Table 41 on page 62)."]
        gdo0_cfg @ 0..6,
    }
    #[doc = "RX FIFO and TX FIFO thresholds"]
    FIFOTHR @ 0x03 = 0b0000_0111 {
        #[doc = "Analog to Digital Converter retention"]
        adc_retention @ 6,
        #[doc = "RX Attenuation, see DN010 for more details"]
        close_in_rx @ 4..6,
        #[doc = "Set the threshold for the TX FIFO and RX FIFO"]
        fifo_thr @ 0..4,
    }
    #[doc = "Sync word, high byte"]
    SYNC1 @ 0x04 = 0b1101_0011 {
        #[doc = "8 MSB of 16-bit sync word"]
        sync @ 0..8,
    }
    #[doc = "Sync word, low byte"]
    SYNC0 @ 0x05 = 0b1001_0001 {
        #[doc = "8 LSB of 16-bit sync word"]
        sync @ 0..8,
    }
    #[doc = "Packet length"]
    PKTLEN @ 0x06 = 0b1111_1111 {
        #[doc = "Packet length if mode is fixed, or max length if variable"]
        packet_length @ 0..8,
    }
    #[doc = "Packet automation control"]
    PKTCTRL1 @ 0x07 = 0b0000_0100 {
        #[doc = "Preamble quality estimator threshold."]
        pqt @ 5..8,
        #[doc = "Enable automatic flush of RX FIFO when CRC is not OK."]
        crc_autoflush @ 3,
        #[doc = "Append RSSI and LQI to RX payload"]
        append_status @ 2,
        #[doc = "Address check configuration of received packages"]
        adr_chk @ 0..2,
    }
    #[doc = "Packet automation control"]
    PKTCTRL0 @ 0x08 = 0b0100_0101 {
        #[doc = "Turn data whitening on / off"]
        white_data @ 6,
        #[doc = "Format of RX and TX data"]
        pkt_format @ 4..6,
        #[doc = "CRC calculation on / off"]
        crc_en @ 2,
        #[doc = "Packet length configuration"]
        length_config @ 0..2,
    }
    #[doc = "Device address"]
    ADDR @ 0x09 = 0b0000_0000 {
        #[doc = "Address used for packet filtration"]
        device_addr @ 0..8,
    }
    #[doc = "Channel number"]
    CHANNR @ 0x0A = 0b0000_0000 {
        #[doc = "Channel number, which is multiplied by the channel spacing setting and added to the base frequency."]
        chan @ 0..8,
    }
    #[doc = "Frequency synthesizer control"]
    FSCTRL1 @ 0x0B = 0b0000_1111 {
        #[doc = "The desired IF frequency to employ in RX"]
        freq_if @ 0..5,
    }
    #[doc = "Frequency synthesizer control"]
    FSCTRL0 @ 0x0C = 0b0000_0000 {
        #[doc = "Frequency offset added to the base frequency before being used by the frequency synthesizer. (2-comp)"]
        freqoff @ 0..8,
    }
    #[doc = "Frequency control word, high byte"]
    FREQ2 @ 0x0D = 0b0001_1110 {
        #[doc = "FREQ\\[21:16\\] is the base frequency for the frequency synthesiser"]
        freq @ 0..6,
    }
    #[doc = "Frequency control word, middle byte"]
    FREQ1 @ 0x0E = 0b1100_0100 {
        #[doc = "FREQ\\[15:8\\], see FREQ2"]
        freq @ 0..8,
    }
    #[doc = "Frequency control word, low byte"]
    FREQ0 @ 0x0F = 0b1110_1100 {
        #[doc = "FREQ\\[7:0\\], see FREQ2"]
        freq @ 0..8,
    }
    #[doc = "Modem configuration"]
    MDMCFG4 @ 0x10 = 0b1000_1100 {
        #[doc = "Exponent of channel bandwidth"]
        chanbw_e @ 6..8,
        #[doc = "Mantissa of channel bandwidth"]
        chanbw_m @ 4..6,
        #[doc = "Exponent of symbol rate"]
        drate_e @ 0..4,
    }
    #[doc = "Modem configuration"]
    MDMCFG3 @ 0x11 = 0b0010_0010 {
        #[doc = "Mantissa of symbol rate"]
        drate_m @ 0..8,
    }
    #[doc = "Modem configuration"]
    MDMCFG2 @ 0x12 = 0b0000_0010 {
        #[doc = "Disable digital DC blocking filter before demodulator"]
        dem_dcfilt_off @ 7,
        #[doc = "The modulation format of the radio signal"]
        mod_format @ 4..7,
        #[doc = "Enables Manchester encoding/decoding"]
        manchester_en @ 3,
        #[doc = "Combined sync-word qualifier mode"]
        sync_mode @ 0..3,
    }
    #[doc = "Modem configuration"]
    MDMCFG1 @ 0x13 = 0b0010_0010 {
        #[doc = "Enable Forward Error Correction"]
        fec_en @ 7,
        #[doc = "Sets the minimum number of preamble bytes to be transmitted"]
        num_preamble @ 4..7,
        #[doc = "Exponent of channel spacing"]
        chanspc_e @ 0..2,
    }
    #[doc = "Modem configuration"]
    MDMCFG0 @ 0x14 = 0b1111_1000 {
        #[doc = "Mantissa of channel spacing"]
        chanspc_m @ 0..8,
    }
    #[doc = "Modem deviation setting"]
    DEVIATN @ 0x15 = 0b0100_0111 {
        #[doc = "Exponent of deviation"]
        deviation_e @ 4..7,
        #[doc = "Mantissa of deviation"]
        deviation_m @ 0..3,
    }
    #[doc = "Main Radio Cntrl State Machine config"]
    MCSM2 @ 0x16 = 0b0000_0111 {
        #[doc = "Direct RX termination based on RSSI measurement"]
        rx_time_rssi @ 4,
        #[doc = "When RX_TIME expires, check sync_word (0), or either sync_word/PQI (1)"]
        rx_time_qual @ 3,
        #[doc = "Timeout for sync word search in RX for both WOR mode and normal RX operation."]
        rx_time @ 0..3,
    }
    #[doc = "Main Radio Cntrl State Machine config"]
    MCSM1 @ 0x17 = 0b0011_0000 {
        #[doc = "Selects CCA_MODE; Reflected in CCA signal"]
        cca_mode @ 4..6,
        #[doc = "Select what should happen when a packet has been received"]
        rxoff_mode @ 2..4,
        #[doc = "Select what should happen when a packet has been sent"]
        txoff_mode @ 0..2,
    }
    #[doc = "Main Radio Cntrl State Machine config"]
    MCSM0 @ 0x18 = 0b0000_0100 {
        #[doc = "Automatically calibrate when going to RX or TX, or back to IDLE"]
        fs_autocal @ 4..6,
        #[doc = "Programs the number of times the six-bit ripple counter must expire after XOSC has stabilized before CHP_RDYn goes low"]
        po_timeout @ 2..4,
        #[doc = "Enables the pin radio control option"]
        pin_ctrl_en @ 1,
        #[doc = "Force the XOSC to stay on in the SLEEP state"]
        xosc_force_on @ 0,
    }
    #[doc = "Frequency Offset Compensation config"]
    FOCCFG @ 0x19 = 0b0011_0110 {
        #[doc = "If set, the demodulator freezes the frequency offset compensation and clock recovery feedback loops until the CS signal goes high"]
        foc_bs_cs_gate @ 5,
        #[doc = "The frequency compensation loop gain to be used before a sync word is detected"]
        foc_pre_k @ 3..5,
        #[doc = "The frequency compensation loop gain to be used after a sync word is detected"]
        foc_post_k @ 2,
        #[doc = "The saturation point for the frequency offset compensation algorithm"]
        foc_limit @ 0..2,
    }
    #[doc = "Bit Synchronization configuration"]
    BSCFG @ 0x1A = 0b0110_1100 {
        #[doc = "The clock recovery feedback loop integral gain to be used before a sync word is detected"]
        bs_pre_ki @ 6..8,
        #[doc = "The clock recovery feedback loop proportional gain to be used before a sync word is detected"]
        bs_pre_kp @ 4..6,
        #[doc = "The clock recovery feedback loop integral gain to be used after a sync word is detected"]
        bs_post_ki @ 3,
        #[doc = "The clock recovery feedback loop proportional gain to be used after a sync word is detected"]
        bs_post_kp @ 2,
        #[doc = "The saturation point for the data rate offset compensation algorithm"]
        bs_limit @ 0..2,
    }
    #[doc = "AGC control"]
    AGCCTRL2 @ 0x1B = 0b0000_0011 {
        #[doc = "Reduces the maximum allowable DVGA gain"]
        max_dvga_gain @ 6..8,
        #[doc = "Sets the maximum allowable LNA + LNA 2 gain relative to the maximum possible gain"]
        max_lna_gain @ 3..6,
        #[doc = "These bits set the target value for the averaged amplitude from the digital channel filter"]
        magn_target @ 0..3,
    }
    #[doc = "AGC control"]
    AGCCTRL1 @ 0x1C = 0b0100_0000 {
        #[doc = "Selects between two different strategies for LNA and LNA 2 gain adjustment"]
        agc_lna_priority @ 6,
        #[doc = "Sets the relative change threshold for asserting carrier sense"]
        carrier_sense_rel_thr @ 4..6,
        #[doc = "Sets the absolute RSSI threshold for asserting carrier sense."]
        carrier_sense_abs_thr @ 0..4,
    }
    #[doc = "AGC control"]
    AGCCTRL0 @ 0x1D = 0b1001_0001 {
        #[doc = "Sets the level of hysteresis on the magnitude deviation"]
        hyst_level @ 6..8,
        #[doc = "Sets the number of channel filter samples from a gain adjustment has been made until the AGC algorithm starts accumulating new samples"]
        wait_time @ 4..6,
        #[doc = "Control when the AGC gain should be frozen"]
        agc_freeze @ 2..4,
        #[doc = "Filter length, in relation to MOD_FORMAT"]
        filter_length @ 0..2,
    }
    #[doc = "High byte Event 0 timeout"]
    WOREVT1 @ 0x1E = 0b1000_0111 {
        #[doc = "High byte of EVENT0 timeout register"]
        event @ 0..8,
    }
    #[doc = "Low byte Event 0 timeout"]
    WOREVT0 @ 0x1F = 0b0110_1011 {
        #[doc = "Low byte of EVENT0 timeout register"]
        event @ 0..8,
    }
    #[doc = "Wake On Radio control"]
    WORCTRL @ 0x20 = 0b1111_1000 {
        #[doc = "Power down signal to RC oscillator"]
        rc_pd @ 7,
        #[doc = "Timeout setting from register block"]
        event @ 4..7,
        #[doc = "Enables (1) or disables (0) the RC oscillator calibration"]
        rc_cal @ 3,
        #[doc = "Controls the Event 0 resolution as well as maximum timeout of the WOR module and maximum timeout under normal RX operation"]
        wor_res @ 0..2,
    }
    #[doc = "Front end RX configuration"]
    FREND1 @ 0x21 = 0b0101_0110 {
        #[doc = "Adjusts front-end LNA PTAT current output"]
        lna_current @ 6..8,
        #[doc = "Adjusts front-end PTAT outputs"]
        lna2mix_current @ 4..6,
        #[doc = "Adjusts current in RX LO buffer (LO input to mixer)"]
        lodiv_buf_current_rx @ 2..4,
        #[doc = "Adjusts current in mixer"]
        mix_current @ 0..2,
    }
    #[doc = "Front end TX configuration"]
    FREND0 @ 0x22 = 0b0001_0000 {
        #[doc = "Adjusts current TX LO buffer (input to PA)"]
        lodiv_buf_current_tx @ 4..6,
        #[doc = "Selects PA power setting"]
        pa_power @ 0..3,
    }
    #[doc = "Frequency synthesizer calibration"]
    FSCAL3 @ 0x23 = 0b1010_1001 {
        #[doc = "Frequency synthesizer calibration configuration"]
        fscal3 @ 6..8,
        #[doc = "Disable charge pump calibration stage when 0"]
        chp_curr_cal_en @ 4..6,
        #[doc = "Frequency synthesizer calibration result register"]
        fscal3_result @ 0..4,
    }
    #[doc = "Frequency synthesizer calibration"]
    FSCAL2 @ 0x24 = 0b0000_1010 {
        #[doc = "Choose high (1) / low (0) VCO"]
        vco_core_h_en @ 5,
        #[doc = "Frequency synthesizer calibration result register, VCO current calibration result and override value"]
        fscal2 @ 0..5,
    }
    #[doc = "Frequency synthesizer calibration"]
    FSCAL1 @ 0x25 = 0b0010_0000 {
        #[doc = "Frequency synthesizer calibration result register, capacitor array setting for VCO coarse tuning"]
        fscal1 @ 0..6,
    }
    #[doc = "Frequency synthesizer calibration"]
    FSCAL0 @ 0x26 = 0b0000_1101 {
        #[doc = "Frequency synthesizer calibration control"]
        fscal0 @ 0..7,
    }
    #[doc = "RC oscillator configuration"]
    RCCTRL1 @ 0x27 = 0b0100_0001 {
        #[doc = "RC oscillator configuration"]
        rcctrl1 @ 0..7,
    }
    #[doc = "RC oscillator configuration"]
    RCCTRL0 @ 0x28 = 0b0000_0000 {
        #[doc = "RC oscillator configuration."]
        rcctrl0 @ 0..7,
    }
    #[doc = "Frequency synthesizer cal control"]
    FSTEST @ 0x29 = 0b0101_1001 {
        #[doc = "For test only. Do not write to this register."]
        fstest @ 0..8,
    }
    #[doc = "Production test"]
    PTEST @ 0x2A = 0b0111_1111 {
        #[doc = "Writing 0xBF to this register makes the on-chip temperature sensor available in the IDLE state"]
        ptest @ 0..8,
    }
    #[doc = "AGC test"]
    AGCTEST @ 0x2B = 0b0011_1111 {
        #[doc = "For test only. Do not write to this register"]
        agctest @ 0..8,
    }
    #[doc = "Various test settings"]
    TEST2 @ 0x2C = 0b1000_1000 {
        #[doc = "The value to use in this register is given by the SmartRF Studio software"]
        test2 @ 0..8,
    }
    #[doc = "Various test settings"]
    TEST1 @ 0x2D = 0b0011_0001 {
        #[doc = "The value to use in this register is given by the SmartRF Studio software"]
        test1 @ 0..8,
    }
    #[doc = "Various test settings"]
    TEST0 @ 0x2E = 0b0000_1011 {
        #[doc = "The value to use in this register is given by the SmartRF Studio software"]
        test0_1 @ 2..8,
        #[doc = "Enable VCO selection calibration stage when 1"]
        vco_sel_cal_en @ 1,
        #[doc = "The value to use in this register is given by the SmartRF Studio software"]
        test0_0 @ 0,
    }
});
