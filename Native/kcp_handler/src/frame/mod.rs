use gdnative::{godot_print, prelude::PoolArray};

#[derive(Debug, Clone)]
pub struct DataFrame {
    pub conv: u32,
    time: u32,
    event: Event,
}

#[derive(Debug, Clone)]
pub enum Event {
    Msg(String),
    Transation {
        id: i32,
        translation: (f32, f32, f32),
    },
}

impl DataFrame {
    pub fn new(conv: u32, time: u32, event: Event) -> Self {
        DataFrame { conv, time, event }
    }

    pub fn handshake(conv: u32, time: u32) -> Self {
        DataFrame {
            conv,
            time,
            event: Event::Msg("handshake".to_string()),
        }
    }

    pub fn default() -> Self {
        DataFrame {
            conv: 0,
            time: 0,
            event: Event::Transation {
                id: 1,
                translation: (0.0, 0.0, 0.0),
            },
        }
    }
    /// todo!!!
    pub fn to_raw(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        let conv = self.conv.to_ne_bytes();
        v.extend_from_slice(&conv);
        let time = self.time.to_ne_bytes();
        v.extend_from_slice(&time);

        match self.event {
            Event::Msg(ref msg) => {
                v.push(0);
                v.push(msg.len() as u8);
                v.extend_from_slice(msg.as_bytes());
            }
            Event::Transation {
                id,
                translation: (x, y, z),
            } => {
                v.push(1);
                let id = id.to_ne_bytes();
                v.extend_from_slice(&id);
                let x = x.to_ne_bytes();
                v.extend_from_slice(&x);
                let y = y.to_ne_bytes();
                v.extend_from_slice(&y);
                let z = z.to_ne_bytes();
                v.extend_from_slice(&z);
            }
        }
        v
    }
}

impl From<DataFrame> for PoolArray<u8> {
    fn from(data_frame: DataFrame) -> Self {
        PoolArray::from_vec(data_frame.to_raw())
    }
}

impl From<PoolArray<u8>> for DataFrame {
    fn from(pool_array: PoolArray<u8>) -> Self {
        let raw = pool_array.to_vec();
        let data_frame: DataFrame = raw.into();
        data_frame
    }
}

impl From<DataFrame> for Vec<u8> {
    fn from(data_frame: DataFrame) -> Self {
        data_frame.to_raw()
    }
}

impl From<Vec<u8>> for DataFrame {
    fn from(raw: Vec<u8>) -> Self {
        let conv = u32::from_ne_bytes([raw[0], raw[1], raw[2], raw[3]]);
        let time = u32::from_ne_bytes([raw[4], raw[5], raw[6], raw[7]]);
        let event = match raw[8] {
            0 => {
                let len = raw[9] as usize;
                let msg = String::from_utf8(raw[10..10 + len].to_vec()).unwrap();
                Event::Msg(msg)
            }
            1 => {
                let id = i32::from_ne_bytes([raw[9], raw[10], raw[11], raw[12]]);
                let x = f32::from_ne_bytes([raw[13], raw[14], raw[15], raw[16]]);
                let y = f32::from_ne_bytes([raw[17], raw[18], raw[19], raw[20]]);
                let z = f32::from_ne_bytes([raw[21], raw[22], raw[23], raw[24]]);
                Event::Transation {
                    id,
                    translation: (x, y, z),
                }
            }
            _ => panic!("invalid event"),
        };
        DataFrame { conv, time, event }
    }
}
