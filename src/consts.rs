#![allow(unsafe_code, dead_code, non_camel_case_types, non_snake_case, unused)]

pub const PI: f64 = 3.141592653589793;
pub const E: f64 = 2.71828;
pub const TAU: f64 = 6.28318530717958647692528676655900576839433879875021164194988918461563281257241799;
pub const NAN: f64 = 0.0 / 0.0;
pub const INF: f64 = 1.0 / 0.0;
pub const NULL: u8 = 0x00;


pub const ELLIPSIS: &str = "...";
pub const EMPTY: &str = "";
pub const SPACE: &str = " ";
pub const TAB: &str = "\t";
pub const NEWLINE: &str = "\n";
pub const CR: &str = "\r";
pub const LF: &str = "\n";
pub const CRLF: &str = "\r\n";
pub const NULL_CHAR: &str = "\0";
pub const NULL_BYTE: &str = "\0";
pub const NULL_WORD: &str = "\0\0";
pub const NULL_DWORD: &str = "\0\0\0\0";
pub const NULL_QWORD: &str = "\0\0\0\0\0\0\0\0";
pub const NULL_FLOAT: &str = "\0\0\0\0";
pub const NULL_DOUBLE: &str = "\0\0\0\0\0\0\0\0";
pub const NULL_BOOL: &str = "\0";
use crate::types::*;





// We'll use some linux kernel consts for simplicity


//PCI Constants
pub mod pci {

    use crate::types::{PciClass, PciVendor};


    pub const PCI_CLASS_NOT_DEFINED: PciClass = 0x000000;
    pub const PCI_CLASS_NOT_DEFINED_VGA: PciClass = 0x000100;

    pub const PCI_CLASS_STORAGE_SCSI: PciClass = 0x010000;
    pub const PCI_CLASS_STORAGE_IDE: PciClass = 0x010100;
    pub const PCI_CLASS_STORAGE_FLOPPY: PciClass = 0x010200;
    pub const PCI_CLASS_STORAGE_IPI: PciClass = 0x010300;
    pub const PCI_CLASS_STORAGE_RAID: PciClass = 0x010400;
    pub const PCI_CLASS_STORAGE_SATA: PciClass = 0x010600;
    pub const PCI_CLASS_STORAGE_SATA_AHCI: PciClass = 0x010601;
    pub const PCI_CLASS_STORAGE_SAS: PciClass = 0x010700;
    pub const PCI_CLASS_STORAGE_EXPRESS: PciClass = 0x010802;
    pub const PCI_CLASS_STORAGE_OTHER: PciClass = 0x018000;
    pub const PCI_CLASS_STORAGE_RVA: PciClass = 0x010500;

    pub const PCI_CLASS_NETWORK_ETHERNET: PciClass = 0x020000;
    pub const PCI_CLASS_NETWORK_TOKEN_RING: PciClass = 0x020100;
    pub const PCI_CLASS_NETWORK_FDDI: PciClass = 0x020200;
    pub const PCI_CLASS_NETWORK_ATM: PciClass = 0x020300;
    pub const PCI_CLASS_NETWORK_OTHER: PciClass = 0x028000;

    pub const PCI_CLASS_DISPLAY_VGA: PciClass = 0x030000;
    pub const PCI_CLASS_DISPLAY_XGA: PciClass = 0x030100;
    pub const PCI_CLASS_DISPLAY_3D: PciClass = 0x030200;
    pub const PCI_CLASS_DISPLAY_OTHER: PciClass = 0x038000;

    pub const PCI_CLASS_MULTIMEDIA_VIDEO: PciClass = 0x040000;
    pub const PCI_CLASS_MULTIMEDIA_AUDIO: PciClass = 0x040100;
    pub const PCI_CLASS_MULTIMEDIA_PHONE: PciClass = 0x040200;
    pub const PCI_CLASS_MUTIMEDIA_HD_AUDIO: PciClass = 0x040300;
    pub const PCI_CLASS_MULTIMEDIA_OTHER: PciClass = 0x048000;

    pub const PCI_CLASS_MEMORY_RAM: PciClass = 0x050000;
    pub const PCI_CLASS_MEMORY_FLASH: PciClass = 0x050100;
    pub const PCI_CLASS_MEMORY_CXL: PciClass = 0x050200;
    pub const PCI_CLASS_MEMORY_SRAM: PciClass = 0x050300;
    pub const PCI_CLASS_MEMORY_DRAM: PciClass = 0x050400;
    pub const PCI_CLASS_MEMORY_OTHER: PciClass = 0x058000;

    pub const PCI_CLASS_BRIDGE_HOST: PciClass = 0x060000;
    pub const PCI_CLASS_BRIDGE_ISA: PciClass = 0x060100;
    pub const PCI_CLASS_BRIDGE_EISA: PciClass = 0x060200;
    pub const PCI_CLASS_BRIDGE_MC: PciClass = 0x060300;
    pub const PCI_CLASS_BRIDGE_PCI_NORMAL: PciClass = 0x060400;
    pub const PCI_CLASS_BRIDGE_PCI_SUBTRACTIVE: PciClass = 0x060401;
    pub const PCI_CLASS_BRIDGE_PCMIA: PciClass = 0x060500;
    pub const PCI_CLASS_BRIDGE_NUBUS: PciClass = 0x060600;
    pub const PCI_CLASS_BRIDGE_CARDBUS: PciClass = 0x060700;
    pub const PCI_CLASS_BRIDGE_RACEWAY: PciClass = 0x060800;
    pub const PCI_CLASS_BRIDGE_OTHER: PciClass = 0x068000;

    pub const PCI_CLASS_COMMUNICATION_SERIAL: PciClass = 0x070000;
    pub const PCI_CLASS_COMMUNICATION_PARALLEL: PciClass = 0x070100;
    pub const PCI_CLASS_COMMUNICATION_MULTISERIAL: PciClass = 0x070200;
    pub const PCI_CLASS_COMMUNICATION_MODEM: PciClass = 0x070300;
    pub const PCI_CLASS_COMMUNICATION_OTHER: PciClass = 0x078000;

    pub const PCI_CLASS_SYSTEM_PIC: PciClass = 0x080000;
    pub const PCI_CLASS_SYSTEM_PIC_IOAPIC: PciClass = 0x080010;
    pub const PCI_CLASS_SYSTEM_PIC_IOXAPIC: PciClass = 0x080020;
    pub const PCI_CLASS_SYSTEM_DMA: PciClass = 0x080100;
    pub const PCI_CLASS_SYSTEM_TIMER: PciClass = 0x080200;
    pub const PCI_CLASS_SYSTEM_RTC: PciClass = 0x080300;
    pub const PCI_CLASS_SYSTEM_PCI_HOTPLUG: PciClass = 0x080400;
    pub const PCI_CLASS_SYSTEM_SDHCI: PciClass = 0x080500;
    pub const PCI_CLASS_SYSTEM_RCEC: PciClass = 0x080700;
    pub const PCI_CLASS_SYSTEM_OTHER: PciClass = 0x088000;

    pub const PCI_CLASS_INPUT_KEYBOARD: PciClass = 0x090000;
    pub const PCI_CLASS_INPUT_PEN: PciClass = 0x090100;
    pub const PCI_CLASS_INPUT_MOUSE: PciClass = 0x090200;
    pub const PCI_CLASS_INPUT_SCANNER: PciClass = 0x090300;
    pub const PCI_CLASS_INPUT_GAMEPORT: PciClass = 0x090400;
    pub const PCI_CLASS_INPUT_OTHER: PciClass = 0x098000;

    pub const PCI_CLASS_DOCKING_GENERIC: PciClass = 0x0a0000;
    pub const PCI_CLASS_DOCKING_OTHER: PciClass = 0x0a8000;

    pub const PCI_CLASS_PROCESSOR_386: PciClass = 0x0b0000;
    pub const PCI_CLASS_PROCESSOR_486: PciClass = 0x0b0100;
    pub const PCI_CLASS_PROCESSOR_PENTIUM: PciClass = 0x0b0200;
    pub const PCI_CLASS_PROCESSOR_ALPHA: PciClass = 0x0b1000;
    pub const PCI_CLASS_PROCESSOR_POWERPC: PciClass = 0x0b2000;
    pub const PCI_CLASS_PROCESSOR_MIPS: PciClass = 0x0b4000;
    pub const PCI_CLASS_PROCESSOR_CO: PciClass = 0x0b5000;

    pub const PCI_CLASS_SERIAL_FIREWIRE: PciClass = 0x0c0000;
    pub const PCI_CLASS_SERIAL_FIREWIRE_OHCI: PciClass = 0x0c0010;
    pub const PCI_CLASS_SERIAL_ACCESS: PciClass = 0x0c0100;
    pub const PCI_CLASS_SERIAL_SSA: PciClass = 0x0c0200;
    pub const PCI_CLASS_SERIAL_USB_UHCI: PciClass = 0x0c0300;
    pub const PCI_CLASS_SERIAL_USB_OHCI: PciClass = 0x0c0310;
    pub const PCI_CLASS_SERIAL_USB_EHCI: PciClass = 0x0c0320;
    pub const PCI_CLASS_SERIAL_USB_XHCI: PciClass = 0x0c0330;
    pub const PCI_CLASS_SERIAL_USB_CDNS: PciClass = 0x0c0380;
    pub const PCI_CLASS_SERIAL_USB_DEVICE: PciClass = 0x0c03fe;
    pub const PCI_CLASS_SERIAL_FIBER: PciClass = 0x0c0400;
    pub const PCI_CLASS_SERIAL_SMBUS: PciClass = 0x0c0500;
    pub const PCI_CLASS_SERIAL_IPMI_SMIC: PciClass = 0x0c0700;
    pub const PCI_CLASS_SERIAL_IPMI_KCS: PciClass = 0x0c0701;
    pub const PCI_CLASS_SERIAL_IPMI_BT: PciClass = 0x0c0702;

    pub const PCI_CLASS_WIRELESS_RF_CONTROLLER: PciClass = 0x0d1000;
    pub const PCI_CLASS_WIRELESS_WHCI: PciClass = 0x0d1010;

    pub const PCI_CLASS_INTELLIGENT_I2O: PciClass = 0x0e0000;

    pub const PCI_CLASS_SATELLITE_TV: PciClass = 0x0f0000;
    pub const PCI_CLASS_SATELLITE_AUDIO: PciClass = 0x0f0100;
    pub const PCI_CLASS_SATELLITE_VOICE: PciClass = 0x0f0300;
    pub const PCI_CLASS_SATELLITE_DATA: PciClass = 0x0f0400;

    pub const PCI_CLASS_CRYPT_NETWORK: PciClass = 0x100000;
    pub const PCI_CLASS_CRYPT_ENTERTAINMENT: PciClass = 0x100100;
    pub const PCI_CLASS_CRYPT_OTHER: PciClass = 0x108000;

    pub const PCI_CLASS_SP_DPIO: PciClass = 0x110000;
    pub const PCI_CLASS_SP_OTHER: PciClass = 0x118000;

    pub const PCI_CLASS_ACCELERATOR_PROCESSING: PciClass = 0x120000;

    pub const PCI_CLASS_OTHERS: PciClass = 0xff0000;


    pub const PCI_VENDOR_ID_PCI_SIG: PciVendor = 0x0001;
    pub const PCI_VENDOR_ID_LOONGSON: PciVendor = 0x0014;
    pub const PCI_VENDOR_ID_SOLIDIGM: PciVendor = 0x025e;
    pub const PCI_VENDOR_ID_TTTECH: PciVendor = 0x0357;
    pub const PCI_VENDOR_ID_DYNALINK: PciVendor = 0x0675;
    pub const PCI_VENDOR_ID_UBIQUITI: PciVendor = 0x0777;
    pub const PCI_VENDOR_ID_BERKOM: PciVendor = 0x0871;
    pub const PCI_VENDOR_ID_ITTIM: PciVendor = 0x0b48;
    pub const PCI_VENDOR_ID_COMPAQ: PciVendor = 0x0e11;
    pub const PCI_VENDOR_ID_LSI_LOGIC: PciVendor = 0x1000;
    pub const PCI_VENDOR_ID_ATI: PciVendor = 0x1002;
    pub const PCI_VENDOR_ID_VLSI: PciVendor = 0x1004;
    pub const PCI_VENDOR_ID_ADL: PciVendor = 0x1005;
    pub const PCI_VENDOR_ID_NS: PciVendor = 0x100b;
    pub const PCI_VENDOR_ID_TSENG: PciVendor = 0x100c;
    pub const PCI_VENDOR_ID_WEITEK: PciVendor = 0x100e;
    pub const PCI_VENDOR_ID_DEC: PciVendor = 0x1011;
    pub const PCI_VENDOR_ID_CIRRUS: PciVendor = 0x1013;
    pub const PCI_VENDOR_ID_IBM: PciVendor = 0x1014;
    pub const PCI_VENDOR_ID_UNISYS: PciVendor = 0x1018;
    pub const PCI_VENDOR_ID_COMPEX2: PciVendor = 0x101a;
    pub const PCI_VENDOR_ID_WD: PciVendor = 0x101c;
    pub const PCI_VENDOR_ID_AMI: PciVendor = 0x101e;
    pub const PCI_VENDOR_ID_AMD: PciVendor = 0x1022;
    pub const PCI_VENDOR_ID_TRIDENT: PciVendor = 0x1023;
    pub const PCI_VENDOR_ID_AI: PciVendor = 0x1025;
    pub const PCI_VENDOR_ID_DELL: PciVendor = 0x1028;
    pub const PCI_VENDOR_ID_MATROX: PciVendor = 0x102B;
    pub const PCI_VENDOR_ID_MOBILITY_ELECTRONICS: PciVendor = 0x14f2;
    pub const PCI_VENDOR_ID_CT: PciVendor = 0x102c;
    pub const PCI_VENDOR_ID_MIRO: PciVendor = 0x1031;
    pub const PCI_VENDOR_ID_NEC: PciVendor = 0x1033;
    pub const PCI_VENDOR_ID_FD: PciVendor = 0x1036;
    pub const PCI_VENDOR_ID_SI: PciVendor = 0x1039;
    pub const PCI_VENDOR_ID_HP: PciVendor = 0x103c;
    pub const PCI_VENDOR_ID_HP_3PAR: PciVendor = 0x1590;
    pub const PCI_VENDOR_ID_PCTECH: PciVendor = 0x1042;
    pub const PCI_VENDOR_ID_ASUSTEK: PciVendor = 0x1043;
    pub const PCI_VENDOR_ID_DPT: PciVendor = 0x1044;
    pub const PCI_VENDOR_ID_OPTI: PciVendor = 0x1045;
    pub const PCI_VENDOR_ID_ELSA: PciVendor = 0x1048;
    pub const PCI_VENDOR_ID_STMICRO: PciVendor = 0x104A;
    pub const PCI_VENDOR_ID_BUSLOGIC: PciVendor = 0x104B;
    pub const PCI_VENDOR_ID_TI: PciVendor = 0x104c;
    pub const PCI_VENDOR_ID_SONY: PciVendor = 0x104d;
    pub const PCI_VENDOR_ID_WINBOND2: PciVendor = 0x1050;
    pub const PCI_VENDOR_ID_ANIGMA: PciVendor = 0x1051;
    pub const PCI_VENDOR_ID_EFAR: PciVendor = 0x1055;
    pub const PCI_VENDOR_ID_MOTOROLA: PciVendor = 0x1057;
    pub const PCI_VENDOR_ID_PROMISE: PciVendor = 0x105a;
    pub const PCI_VENDOR_ID_FOXCONN: PciVendor = 0x105b;
    pub const PCI_VENDOR_ID_UMC: PciVendor = 0x1060;
    pub const PCI_VENDOR_ID_PICOPOWER: PciVendor = 0x1066;
    pub const PCI_VENDOR_ID_MYLEX: PciVendor = 0x1069;
    pub const PCI_VENDOR_ID_APPLE: PciVendor = 0x106b;
    pub const PCI_VENDOR_ID_YAMAHA: PciVendor = 0x1073;
    pub const PCI_VENDOR_ID_QLOGIC: PciVendor = 0x1077;
    pub const PCI_VENDOR_ID_CYRIX: PciVendor = 0x1078;
    pub const PCI_VENDOR_ID_CONTAQ: PciVendor = 0x1080;
    pub const PCI_VENDOR_ID_OLICOM: PciVendor = 0x108d;
    pub const PCI_VENDOR_ID_SUN: PciVendor = 0x108e;
    pub const PCI_VENDOR_ID_NI: PciVendor = 0x1093;
    pub const PCI_VENDOR_ID_CMD: PciVendor = 0x1095;
    pub const PCI_VENDOR_ID_BROOKTREE: PciVendor = 0x109e;
    pub const PCI_VENDOR_ID_SGI: PciVendor = 0x10a9;
    pub const PCI_VENDOR_ID_WINBOND: PciVendor = 0x10ad;
    pub const PCI_VENDOR_ID_PLX: PciVendor = 0x10b5;
    pub const PCI_VENDOR_ID_MADGE: PciVendor = 0x10b6;
    pub const PCI_VENDOR_ID_3COM: PciVendor = 0x10b7;
    pub const PCI_VENDOR_ID_AL: PciVendor = 0x10b9;
    pub const PCI_VENDOR_ID_NEOMAGIC: PciVendor = 0x10c8;
    pub const PCI_VENDOR_ID_TCONRAD: PciVendor = 0x10da;
    pub const PCI_VENDOR_ID_ROHM: PciVendor = 0x10db;
    pub const PCI_VENDOR_ID_NVIDIA: PciVendor = 0x10de;
    pub const PCI_VENDOR_ID_IMS: PciVendor = 0x10e0;
    pub const PCI_VENDOR_ID_AMCC: PciVendor = 0x10e8;
    pub const PCI_VENDOR_ID_AMPERE: PciVendor = 0x1def;
    pub const PCI_VENDOR_ID_INTERG: PciVendor = 0x10ea;
    pub const PCI_VENDOR_ID_REALTEK: PciVendor = 0x10ec;
    pub const PCI_VENDOR_ID_XILINX: PciVendor = 0x10ee;
    pub const PCI_VENDOR_ID_INIT: PciVendor = 0x1101;
    pub const PCI_VENDOR_ID_CREATIVE: PciVendor = 0x1102;
    pub const PCI_VENDOR_ID_TTI: PciVendor = 0x1103;
    pub const PCI_VENDOR_ID_SIGMA: PciVendor = 0x1105;
    pub const PCI_VENDOR_ID_VIA: PciVendor = 0x1106;
    pub const PCI_VENDOR_ID_SIEMENS: PciVendor = 0x110A;
    pub const PCI_VENDOR_ID_VORTEX: PciVendor = 0x1119;
    pub const PCI_VENDOR_ID_EF: PciVendor = 0x111a;
    pub const PCI_VENDOR_ID_IDT: PciVendor = 0x111d;
    pub const PCI_VENDOR_ID_FORE: PciVendor = 0x1127;
    pub const PCI_VENDOR_ID_PHILIPS: PciVendor = 0x1131;
    pub const PCI_VENDOR_ID_EICON: PciVendor = 0x1133;
    pub const PCI_VENDOR_ID_CISCO: PciVendor = 0x1137;
    pub const PCI_VENDOR_ID_ZIATECH: PciVendor = 0x1138;
    pub const PCI_VENDOR_ID_SYSKONNECT: PciVendor = 0x1148;
    pub const PCI_VENDOR_ID_DIGI: PciVendor = 0x114f;
    pub const PCI_VENDOR_ID_XIRCOM: PciVendor = 0x115d;
    pub const PCI_VENDOR_ID_SERVERWORKS: PciVendor = 0x1166;
    pub const PCI_VENDOR_ID_ALTERA: PciVendor = 0x1172;
    pub const PCI_VENDOR_ID_SBE: PciVendor = 0x1176;
    pub const PCI_VENDOR_ID_TOSHIBA: PciVendor = 0x1179;
    pub const PCI_VENDOR_ID_TOSHIBA_2: PciVendor = 0x102f;
    pub const PCI_VENDOR_ID_ATTO: PciVendor = 0x117c;
    pub const PCI_VENDOR_ID_RICOH: PciVendor = 0x1180;
    pub const PCI_VENDOR_ID_DLINK: PciVendor = 0x1186;
    pub const PCI_VENDOR_ID_ARTOP: PciVendor = 0x1191;
    pub const PCI_VENDOR_ID_ZEITNET: PciVendor = 0x1193;
    pub const PCI_VENDOR_ID_FUJITSU_ME: PciVendor = 0x119e;
    pub const PCI_VENDOR_ID_MARVELL: PciVendor = 0x11ab;
    pub const PCI_VENDOR_ID_MARVELL_EXT: PciVendor = 0x1b4b;
    pub const PCI_VENDOR_ID_V3: PciVendor = 0x11b0;
    pub const PCI_VENDOR_ID_ATT: PciVendor = 0x11c1;
    pub const PCI_VENDOR_ID_SPECIALIX: PciVendor = 0x11cb;
    pub const PCI_VENDOR_ID_ANALOG_DEVICES: PciVendor = 0x11d4;
    pub const PCI_VENDOR_ID_ZORAN: PciVendor = 0x11de;
    pub const PCI_VENDOR_ID_COMPEX: PciVendor = 0x11f6;
    pub const PCI_VENDOR_ID_MICROSEMI: PciVendor = 0x11f8;
    pub const PCI_VENDOR_ID_RP: PciVendor = 0x11fe;
    pub const PCI_VENDOR_ID_CYCLADES: PciVendor = 0x120e;
    pub const PCI_VENDOR_ID_O2: PciVendor = 0x1217;
    pub const PCI_VENDOR_ID_3DFX: PciVendor = 0x121a;
    pub const PCI_VENDOR_ID_AVM: PciVendor = 0x1244;
    pub const PCI_VENDOR_ID_STALLION: PciVendor = 0x124d;
    pub const PCI_VENDOR_ID_AT: PciVendor = 0x1259;
    pub const PCI_VENDOR_ID_ASIX: PciVendor = 0x125b;
    pub const PCI_VENDOR_ID_ESS: PciVendor = 0x125d;
    pub const PCI_VENDOR_ID_SATSAGEM: PciVendor = 0x1267;
    pub const PCI_VENDOR_ID_ENSONIQ: PciVendor = 0x1274;
    pub const PCI_VENDOR_ID_TRANSMETA: PciVendor = 0x1279;
    pub const PCI_VENDOR_ID_ROCKWELL: PciVendor = 0x127A;
    pub const PCI_VENDOR_ID_ITE: PciVendor = 0x1283;
    pub const PCI_VENDOR_ID_ALTEON: PciVendor = 0x12ae;
    pub const PCI_VENDOR_ID_NVIDIA_SGS: PciVendor = 0x12d2;
    pub const PCI_VENDOR_ID_PERICOM: PciVendor = 0x12D8;
    pub const PCI_VENDOR_ID_AUREAL: PciVendor = 0x12eb;
    pub const PCI_VENDOR_ID_ELECTRONICDESIGNGMBH: PciVendor = 0x12f8;
    pub const PCI_VENDOR_ID_ESDGMBH: PciVendor = 0x12fe;
    pub const PCI_VENDOR_ID_CB: PciVendor = 0x1307;
    pub const PCI_VENDOR_ID_SIIG: PciVendor = 0x131f;
    pub const PCI_VENDOR_ID_RADISYS: PciVendor = 0x1331;
    pub const PCI_VENDOR_ID_MICRO_MEMORY: PciVendor = 0x1332;
    pub const PCI_VENDOR_ID_DOMEX: PciVendor = 0x134a;
    pub const PCI_VENDOR_ID_INTASHIELD: PciVendor = 0x135a;
    pub const PCI_VENDOR_ID_QUATECH: PciVendor = 0x135C;
    pub const PCI_VENDOR_ID_SEALEVEL: PciVendor = 0x135e;
    pub const PCI_VENDOR_ID_HYPERCOPE: PciVendor = 0x1365;
    pub const PCI_VENDOR_ID_DIGIGRAM: PciVendor = 0x1369;
    pub const PCI_VENDOR_ID_KAWASAKI: PciVendor = 0x136b;
    pub const PCI_VENDOR_ID_CNET: PciVendor = 0x1371;
    pub const PCI_VENDOR_ID_LMC: PciVendor = 0x1376;
    pub const PCI_VENDOR_ID_NETGEAR: PciVendor = 0x1385;
    pub const PCI_VENDOR_ID_APPLICOM: PciVendor = 0x1389;
    pub const PCI_VENDOR_ID_MOXA: PciVendor = 0x1393;
    pub const PCI_VENDOR_ID_CCD: PciVendor = 0x1397;
    pub const PCI_VENDOR_ID_EXAR: PciVendor = 0x13a8;
    pub const PCI_VENDOR_ID_MICROGATE: PciVendor = 0x13c0;
    pub const PCI_VENDOR_ID_3WARE: PciVendor = 0x13C1;
    pub const PCI_VENDOR_ID_IOMEGA: PciVendor = 0x13ca;
    pub const PCI_VENDOR_ID_ABOCOM: PciVendor = 0x13D1;
    pub const PCI_VENDOR_ID_SUNDANCE: PciVendor = 0x13f0;
    pub const PCI_VENDOR_ID_CMEDIA: PciVendor = 0x13f6;
    pub const PCI_VENDOR_ID_ADVANTECH: PciVendor = 0x13fe;
    pub const PCI_VENDOR_ID_MEILHAUS: PciVendor = 0x1402;
    pub const PCI_VENDOR_ID_LAVA: PciVendor = 0x1407;
    pub const PCI_VENDOR_ID_TIMEDIA: PciVendor = 0x1409;
    pub const PCI_VENDOR_ID_ICE: PciVendor = 0x1412;
    pub const PCI_VENDOR_ID_MICROSOFT: PciVendor = 0x1414;
    pub const PCI_VENDOR_ID_OXSEMI: PciVendor = 0x1415;
    pub const PCI_VENDOR_ID_CHELSIO: PciVendor = 0x1425;
    pub const PCI_VENDOR_ID_EDIMAX: PciVendor = 0x1432;
    pub const PCI_VENDOR_ID_ADLINK: PciVendor = 0x144a;
    pub const PCI_VENDOR_ID_SAMSUNG: PciVendor = 0x144d;
    pub const PCI_VENDOR_ID_GIGABYTE: PciVendor = 0x1458;
    pub const PCI_VENDOR_ID_AMBIT: PciVendor = 0x1468;
    pub const PCI_VENDOR_ID_MYRICOM: PciVendor = 0x14c1;
    pub const PCI_VENDOR_ID_MEDIATEK: PciVendor = 0x14c3;
    pub const PCI_VENDOR_ID_TITAN: PciVendor = 0x14D2;
    pub const PCI_VENDOR_ID_PANACOM: PciVendor = 0x14d4;
    pub const PCI_VENDOR_ID_SIPACKETS: PciVendor = 0x14d9;
    pub const PCI_VENDOR_ID_AFAVLAB: PciVendor = 0x14db;
    pub const PCI_VENDOR_ID_AMPLICON: PciVendor = 0x14dc;
    pub const PCI_VENDOR_ID_BCM_GVC: PciVendor = 0x14a4;
    pub const PCI_VENDOR_ID_BROADCOM: PciVendor = 0x14e4;
    pub const PCI_VENDOR_ID_TOPIC: PciVendor = 0x151f;
    pub const PCI_VENDOR_ID_MAINPINE: PciVendor = 0x1522;
    pub const PCI_VENDOR_ID_ENE: PciVendor = 0x1524;
    pub const PCI_VENDOR_ID_SYBA: PciVendor = 0x1592;
    pub const PCI_VENDOR_ID_MORETON: PciVendor = 0x15aa;
    pub const PCI_VENDOR_ID_VMWARE: PciVendor = 0x15ad;
    pub const PCI_VENDOR_ID_ZOLTRIX: PciVendor = 0x15b0;
    pub const PCI_VENDOR_ID_MELLANOX: PciVendor = 0x15b3;
    pub const PCI_VENDOR_ID_DFI: PciVendor = 0x15bd;
    pub const PCI_VENDOR_ID_QUICKNET: PciVendor = 0x15e2;
    pub const PCI_VENDOR_ID_ADDIDATA: PciVendor = 0x15B8;
    pub const PCI_VENDOR_ID_PDC: PciVendor = 0x15e9;
    pub const PCI_VENDOR_ID_FARSITE: PciVendor = 0x1619;
    pub const PCI_VENDOR_ID_ARIMA: PciVendor = 0x161f;
    pub const PCI_VENDOR_ID_BROCADE: PciVendor = 0x1657;
    pub const PCI_VENDOR_ID_SIBYTE: PciVendor = 0x166d;
    pub const PCI_VENDOR_ID_ATHEROS: PciVendor = 0x168c;
    pub const PCI_VENDOR_ID_NETCELL: PciVendor = 0x169c;
    pub const PCI_VENDOR_ID_CENATEK: PciVendor = 0x16CA;
    pub const PCI_VENDOR_ID_SYNOPSYS: PciVendor = 0x16c3;
    pub const PCI_VENDOR_ID_USR: PciVendor = 0x16ec;
    pub const PCI_VENDOR_ID_VITESSE: PciVendor = 0x1725;
    pub const PCI_VENDOR_ID_LINKSYS: PciVendor = 0x1737;
    pub const PCI_VENDOR_ID_ALTIMA: PciVendor = 0x173b;
    pub const PCI_VENDOR_ID_CAVIUM: PciVendor = 0x177d;
    pub const PCI_VENDOR_ID_TECHWELL: PciVendor = 0x1797;
    pub const PCI_VENDOR_ID_BELKIN: PciVendor = 0x1799;
    pub const PCI_VENDOR_ID_RDC: PciVendor = 0x17f3;
    pub const PCI_VENDOR_ID_GLI: PciVendor = 0x17a0;
    pub const PCI_VENDOR_ID_LENOVO: PciVendor = 0x17aa;
    pub const PCI_VENDOR_ID_QCOM: PciVendor = 0x17cb;
    pub const PCI_VENDOR_ID_CDNS: PciVendor = 0x17cd;
    pub const PCI_VENDOR_ID_ARECA: PciVendor = 0x17d3;
    pub const PCI_VENDOR_ID_S2IO: PciVendor = 0x17d5;
    pub const PCI_VENDOR_ID_SITECOM: PciVendor = 0x182d;
    pub const PCI_VENDOR_ID_TOPSPIN: PciVendor = 0x1867;
    pub const PCI_VENDOR_ID_COMMTECH: PciVendor = 0x18f7;
    pub const PCI_VENDOR_ID_SILAN: PciVendor = 0x1904;
    pub const PCI_VENDOR_ID_RENESAS: PciVendor = 0x1912;
    pub const PCI_VENDOR_ID_SOLARFLARE: PciVendor = 0x1924;
    pub const PCI_VENDOR_ID_TDI: PciVendor = 0x192E;
    pub const PCI_VENDOR_ID_NXP: PciVendor = 0x1957;
    pub const PCI_VENDOR_ID_PASEMI: PciVendor = 0x1959;
    pub const PCI_VENDOR_ID_ATTANSIC: PciVendor = 0x1969;
    pub const PCI_VENDOR_ID_JMICRON: PciVendor = 0x197B;
    pub const PCI_VENDOR_ID_KORENIX: PciVendor = 0x1982;
    pub const PCI_VENDOR_ID_HUAWEI: PciVendor = 0x19e5;
    pub const PCI_VENDOR_ID_NETRONOME: PciVendor = 0x19ee;
    pub const PCI_VENDOR_ID_QMI: PciVendor = 0x1a32;
    pub const PCI_VENDOR_ID_AZWAVE: PciVendor = 0x1a3b;
    pub const PCI_VENDOR_ID_REDHAT_QUMRANET: PciVendor = 0x1af4;
    pub const PCI_VENDOR_ID_ASMEDIA: PciVendor = 0x1b21;
    pub const PCI_VENDOR_ID_REDHAT: PciVendor = 0x1b36;
    pub const PCI_VENDOR_ID_WCHIC: PciVendor = 0x1c00;
    pub const PCI_VENDOR_ID_SILICOM_DENMARK: PciVendor = 0x1c2c;
    pub const PCI_VENDOR_ID_AMAZON_ANNAPURNA_LABS: PciVendor = 0x1c36;
    pub const PCI_VENDOR_ID_CIRCUITCO: PciVendor = 0x1cc8;
    pub const PCI_VENDOR_ID_AMAZON: PciVendor = 0x1d0f;
    pub const PCI_VENDOR_ID_ZHAOXIN: PciVendor = 0x1d17;
    pub const PCI_VENDOR_ID_ROCKCHIP: PciVendor = 0x1d87;
    pub const PCI_VENDOR_ID_HYGON: PciVendor = 0x1d94;
    pub const PCI_VENDOR_ID_META: PciVendor = 0x1d9b;
    pub const PCI_VENDOR_ID_FUNGIBLE: PciVendor = 0x1dad;
    pub const PCI_VENDOR_ID_HXT: PciVendor = 0x1dbf;
    pub const PCI_VENDOR_ID_TEKRAM: PciVendor = 0x1de1;
    pub const PCI_VENDOR_ID_RPI: PciVendor = 0x1de4;
    pub const PCI_VENDOR_ID_ALIBABA: PciVendor = 0x1ded;
    pub const PCI_VENDOR_ID_CXL: PciVendor = 0x1e98;
    pub const PCI_VENDOR_ID_TEHUTI: PciVendor = 0x1fc9;
    pub const PCI_VENDOR_ID_SUNIX: PciVendor = 0x1fd4;
    pub const PCI_VENDOR_ID_HINT: PciVendor = 0x3388;
    pub const PCI_VENDOR_ID_3DLABS: PciVendor = 0x3d3d;
    pub const PCI_VENDOR_ID_NETXEN: PciVendor = 0x4040;
    pub const PCI_VENDOR_ID_AKS: PciVendor = 0x416c;
    pub const PCI_VENDOR_ID_WCHCN: PciVendor = 0x4348;
    pub const PCI_VENDOR_ID_ACCESSIO: PciVendor = 0x494f;
    pub const PCI_VENDOR_ID_S3: PciVendor = 0x5333;
    pub const PCI_VENDOR_ID_DUNORD                   : PciVendor = 0x5544;
    pub const PCI_VENDOR_ID_DCI                      : PciVendor = 0x6666;
    pub const PCI_VENDOR_ID_GLENFLY                  : PciVendor = 0x6766;
    pub const PCI_VENDOR_ID_INTEL                    : PciVendor = 0x8086;
    pub const PCI_VENDOR_ID_WANGXUN                  : PciVendor = 0x8088;
    pub const PCI_VENDOR_ID_SCALEMP                  : PciVendor = 0x8686;
    pub const PCI_VENDOR_ID_COMPUTONE                : PciVendor = 0x8e0e;
    pub const PCI_VENDOR_ID_KTI                      : PciVendor = 0x8e2e;
    pub const PCI_VENDOR_ID_ADAPTEC                  : PciVendor = 0x9004;
    pub const PCI_VENDOR_ID_ADAPTEC2                 : PciVendor = 0x9005;
    pub const PCI_VENDOR_ID_HOLTEK                   : PciVendor = 0x9412;
    pub const PCI_VENDOR_ID_NETMOS                   : PciVendor = 0x9710;
    pub const PCI_VENDOR_ID_3COM_2                   : PciVendor = 0xa727;
    pub const PCI_VENDOR_ID_SOLIDRUN                 : PciVendor = 0xd063;
    pub const PCI_VENDOR_ID_DIGIUM                   : PciVendor = 0xd161;
    pub const PCI_VENDOR_ID_TIGERJET                 : PciVendor = 0xe159;
    pub const PCI_VENDOR_ID_XILINX_RME               : PciVendor = 0xea60;
    pub const PCI_VENDOR_ID_XEN                      : PciVendor = 0x5853;
    pub const PCI_VENDOR_ID_OCZ                      : PciVendor = 0x1b85;
    pub const PCI_VENDOR_ID_NCUBE                    : PciVendor = 0x10ff;
}


// FS constants
pub mod fs {
    pub type Offset = u32;
    pub mod flags {
        /// File is opened in append mode.
        pub const O_APPEND: u32 = 0x0000;
        /// Signal-driven I/O is enabled.
        pub const O_ASYNC: u32 = 0x0000;
        /// Close-on-exec flag is set.
        pub const O_CLOEXEC: u32 = 0x0000;
        /// File was created if it didn't already exist.
        pub const O_CREATE: u32 = 0x0000;
        /// Direct I/O is enabled for this file.
        pub const O_DIRECT: u32 = 0x0000;
        /// File must be a directory.
        pub const O_DIRECTORY: u32 = 0x0000;
        /// Like [`O_SYNC`] except metadata is not synced.
        pub const O_DSYNC: u32 = 0x0000;
        /// Ensure that this file is created with the `open(2)` call.
        pub const O_EXCL: u32 = 0x0000;
        /// Large file size enabled (`off64_t` over `off_t`).
        pub const O_LARGEFILE: u32 = 0x0000;
        /// Do not update the file last access time.
        pub const O_NOATIME: u32 = 0x0000;
        /// File should not be used as process's controlling terminal.
        pub const O_NOCTTY: u32 = 0x0000;
        /// If basename of path is a symbolic link, fail open.
        pub const O_NOFOLLOW: u32 = 0x0000;
        /// File is using nonblocking I/O.
        pub const O_NONBLOCK: u32 = 0x0000;
        /// File is using nonblocking I/O.
        pub const O_NDELAY: u32 = 0x0000;
        /// Used to obtain a path file descriptor.
        pub const O_PATH: u32 = 0x0000;
        /// Write operations on this file will flush data and metadata.
        pub const O_SYNC: u32 = 0x0000;
        /// This file is an unnamed temporary regular file.
        pub const O_TMPFILE: u32 = 0x0000;
        /// File should be truncated to length 0.
        pub const O_TRUNC: u32 = 0x0000;
        /// Bitmask for access mode flags.
        ///
        /// # Examples
        ///
        /// ```
        /// use kernel::fs::file;
        /// # fn do_something() {}
        /// # let flags = 0;
        /// if (flags & file::flags::O_ACCMODE) == file::flags::O_RDONLY {
        ///     do_something();
        /// }
        /// ```
        pub const O_ACCMODE: u32 = 0x0000;
        /// File is read only.
        pub const O_RDONLY: u32 = 0x0000;
        /// File is write only.
        pub const O_WRONLY: u32 = 0x0000;
        /// File can be both read and written.
        pub const O_RDWR: u32 = 0x0000;
    }
}


pub mod iommu {
    pub mod prot {
        /// Read access.
        pub const READ: u32 = 0x0000;
        /// Write access.
        pub const WRITE: u32 = 0x0000;
        /// Request cache coherency.
        pub const CACHE: u32 = 0x0000;
        /// Request no-execute permission.
        pub const NOEXEC: u32 = 0x0000;
        /// MMIO peripheral mapping.
        pub const MMIO: u32 = 0x0000;
        /// Privileged mapping.
        pub const PRIVILEGED: u32 = 0x0000;
    }
}


pub mod irq {
    use crate::types::{LONG, ULONG, ULONG_MAX};
    pub struct Flags(ULONG);
    impl Flags {
        /// Use the interrupt line as already configured.
        pub const TRIGGER_NONE: Flags = Flags::new(0x00000000);

        /// The interrupt is triggered when the signal goes from low to high.
        pub const TRIGGER_RISING: Flags = Flags::new(0x00000000);

        /// The interrupt is triggered when the signal goes from high to low.
        pub const TRIGGER_FALLING: Flags = Flags::new(0x00000000);

        /// The interrupt is triggered while the signal is held high.
        pub const TRIGGER_HIGH: Flags = Flags::new(0x00000000);

        /// The interrupt is triggered while the signal is held low.
        pub const TRIGGER_LOW: Flags = Flags::new(0x00000000);

        /// Allow sharing the IRQ among several devices.
        pub const SHARED: Flags = Flags::new(0x00000000);

        /// Set by callers when they expect sharing mismatches to occur.
        pub const PROBE_SHARED: Flags = Flags::new(0x00000000);

        /// Flag to mark this interrupt as timer interrupt.
        pub const TIMER: Flags = Flags::new(0x00000000);

        /// Interrupt is per CPU.
        pub const PERCPU: Flags = Flags::new(0x00000000);

        /// Flag to exclude this interrupt from irq balancing.
        pub const NOBALANCING: Flags = Flags::new(0x00000000);

        /// Interrupt is used for polling (only the interrupt that is registered
        /// first in a shared interrupt is considered for performance reasons).
        pub const IRQPOLL: Flags = Flags::new(0x00000000);

        /// Interrupt is not re-enabled after the hardirq handler finished. Used by
        /// threaded interrupts which need to keep the irq line disabled until the
        /// threaded handler has been run.
        pub const ONESHOT: Flags = Flags::new(0x00000000);

        /// Do not disable this IRQ during suspend. Does not guarantee that this
        /// interrupt will wake the system from a suspended state.
        pub const NO_SUSPEND: Flags = Flags::new(0x00000000);

        /// Force enable it on resume even if [`Flags::NO_SUSPEND`] is set.
        pub const FORCE_RESUME: Flags = Flags::new(0x00000000);

        /// Interrupt cannot be threaded.
        pub const NO_THREAD: Flags = Flags::new(0x00000000);

        /// Resume IRQ early during syscore instead of at device resume time.
        pub const EARLY_RESUME: Flags = Flags::new(0x00000000);

        /// If the IRQ is shared with a [`Flags::NO_SUSPEND`] user, execute this
        /// interrupt handler after suspending interrupts. For system wakeup devices
        /// users need to implement wakeup detection in their interrupt handlers.
        pub const COND_SUSPEND: Flags = Flags::new(0x00000000);

        /// Don't enable IRQ or NMI automatically when users request it. Users will
        /// enable it explicitly by `enable_irq` or `enable_nmi` later.
        pub const NO_AUTOEN: Flags = Flags::new(0x00000000);

        /// Exclude from runnaway detection for IPI and similar handlers, depends on
        /// `PERCPU`.
        pub const NO_DEBUG: Flags = Flags::new(0x00000000);

        pub(crate) fn into_inner(self) -> ULONG {
            self.0
        }

        // Always inline to optimize out error path of `build_assert`.
        #[inline(always)]
        const fn new(value: u32) -> Self {
            assert!(value as u64 <= ULONG_MAX as u64);
            Self(value as ULONG)
        }
    }
}

pub mod mm_virt{
    pub mod flags {
        pub type vm_flags_t = u32;

        /// No flags are set.
        pub const NONE: vm_flags_t = 0x00000000;

        /// Mapping allows reads.
        pub const READ: vm_flags_t = 0x00000000;

        /// Mapping allows writes.
        pub const WRITE: vm_flags_t = 0x00000000;

        /// Mapping allows execution.
        pub const EXEC: vm_flags_t = 0x00000000;

        /// Mapping is shared.
        pub const SHARED: vm_flags_t = 0x00000000;

        /// Mapping may be updated to allow reads.
        pub const MAYREAD: vm_flags_t = 0x00000000;

        /// Mapping may be updated to allow writes.
        pub const MAYWRITE: vm_flags_t = 0x00000000;

        /// Mapping may be updated to allow execution.
        pub const MAYEXEC: vm_flags_t = 0x00000000;

        /// Mapping may be updated to be shared.
        pub const MAYSHARE: vm_flags_t = 0x00000000;

        /// Page-ranges managed without `struct page`, just pure PFN.
        pub const PFNMAP: vm_flags_t = 0x00000000;

        /// Memory mapped I/O or similar.
        pub const IO: vm_flags_t = 0x00000000;

        /// Do not copy this vma on fork.
        pub const DONTCOPY: vm_flags_t = 0x00000000;

        /// Cannot expand with mremap().
        pub const DONTEXPAND: vm_flags_t = 0x00000000;

        /// Lock the pages covered when they are faulted in.
        pub const LOCKONFAULT: vm_flags_t = 0x00000000;

        /// Is a VM accounted object.
        pub const ACCOUNT: vm_flags_t = 0x00000000;

        /// Should the VM suppress accounting.
        pub const NORESERVE: vm_flags_t = 0x00000000;

        /// Huge TLB Page VM.
        pub const HUGETLB: vm_flags_t = 0x00000000;

        /// Synchronous page faults. (DAX-specific)
        pub const SYNC: vm_flags_t = 0x00000000;

        /// Architecture-specific flag.
        pub const ARCH_1: vm_flags_t = 0x00000000;

        /// Wipe VMA contents in child on fork.
        pub const WIPEONFORK: vm_flags_t = 0x00000000;

        /// Do not include in the core dump.
        pub const DONTDUMP: vm_flags_t = 0x00000000;

        /// Not soft dirty clean area.
        pub const SOFTDIRTY: vm_flags_t = 0x00000000;

        /// Can contain `struct page` and pure PFN pages.
        pub const MIXEDMAP: vm_flags_t = 0x00000000;

        /// MADV_HUGEPAGE marked this vma.
        pub const HUGEPAGE: vm_flags_t = 0x00000000;

        /// MADV_NOHUGEPAGE marked this vma.
        pub const NOHUGEPAGE: vm_flags_t = 0x00000000;

        /// KSM may merge identical pages.
        pub const MERGEABLE: vm_flags_t = 0x00000000;
    }
}

pub mod net {
    pub mod phy {
        use crate::types::net::phy::PhyDeviceId;
        use crate::types::Str;

        pub mod flags {
            /// PHY is internal.
            pub const IS_INTERNAL: u32 = 0x0;
            /// PHY needs to be reset after the refclk is enabled.
            pub const RST_AFTER_CLK_EN: u32 = 0x0;
            /// Polling is used to detect PHY status changes.
            pub const POLL_CABLE_TEST: u32 = 0x0;
            /// Don't suspend.
            pub const ALWAYS_CALL_SUSPEND: u32 = 0x0;
        }
        pub trait Driver {
            /// Defines certain other features this PHY supports.
            /// It is a combination of the flags in the [`flags`] module.
            const FLAGS: u32 = 0;

            /// The friendly name of this PHY type.
            const NAME: &'static Str;

            /// This driver only works for PHYs with IDs which match this field.
            /// The default id and mask are zero.
            const PHY_DEVICE_ID: PhyDeviceId = PhyDeviceId::new_with_custom_mask(0, 0);
        }
    }
}