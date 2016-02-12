use ppu::Ppu;
use std::num::Wrapping as W;

const STACK_PAGE        : u16 = 0x0100;

pub struct Memory {
    ram : [u8; 2048],
    ppu : Ppu,
}

impl Memory {
    pub fn new (ppu : Ppu) -> Memory {
        Memory {
            ram : [0;  2048],
            ppu : ppu,
        }
    }

    pub fn load (&self, address: u16) -> u8 {
            if address < 0x2000 {
                self.ram[(address & 0x7ff) as usize]
            } else if address < 0x4000 {
                match (address % 0x2000) & 0x7 {
                    //0 => self.ppu.ppuctrl, En teoria los registros comentados son read only
                    //1 => self.ppu.ppumask,
                    2 => self.ppu.ppustatus,
                    //3 => self.ppu.oamaddr,
                    4 => self.ppu.oamdata,
                    //5 => self.ppu.ppuscroll,
                    //6 => self.ppu.ppuaddr,
                    7 => self.ppu.ppudata,
                    _ => 0 // fuck you.
                }
            } else if address < 0x4020 {
                /* Apu TODO*/
                0 
            } else if address < 0x6000 {
                /* Cartridge expansion ROM the f */
                0
            } else if address < 0x8000 {
                /* SRAM */
                0
            } else /* 0x8000 <= address < 0xC000*/ {
                /* PRG-ROM */
                0
            }
    }

    pub fn store (&mut self, address: u16, value : u8){
        if address < 0x2000 {
            self.ram[(address & 0x7ff) as usize] = value
        } else if address < 0x4000 {
            match (address % 0x2000) & 0x7 {
                0 => self.ppu.ppuctrl = value,
                1 => self.ppu.ppumask = value, 
                //2 => self.ppu.ppustatus = value, Este registro es read only
                3 => self.ppu.oamaddr = value,
                4 => self.ppu.oamdata = value, 
                5 => self.ppu.ppuscroll = value,
                6 => self.ppu.ppuaddr = value,
                7 => self.ppu.ppudata = value,
                _ => self.ppu.ppuctrl = self.ppu.ppuctrl  // epic.
            }
        } else if address < 0x4020 {
            /* Apu TODO*/
             
        } else if address < 0x6000 {
            /* Cartridge expansion ROM the f */
            
        } else if address < 0x8000 {
            /* SRAM */
           
        } else /* 0x8000 <= address < 0xC000*/ {
            /* PRG-ROM */
           
        }
    }

    pub fn load_word(&mut self, address: W<u16>) -> u16 {
        let low = self.load(address.0) as u16;
        (self.load((address + W(1)).0) as u16) << 8 | low
    }

    pub fn store_word(&mut self, address: W<u16>, word: u16) {
        self.store(address.0, (word >> 8) as u8);
        self.store((address + W(1)).0, word as u8);
    }
}
