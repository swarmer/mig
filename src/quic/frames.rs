pub const FRAME_PADDING: u8 = 0x00;
pub const FRAME_RST_STREAM: u8 = 0x01;
pub const FRAME_CONNECTION_CLOSE: u8 = 0x02;
pub const FRAME_GOAWAY: u8 = 0x03;
pub const FRAME_WINDOW_UPDATE: u8 = 0x04;
pub const FRAME_BLOCKED: u8 = 0x05;
pub const FRAME_STOP_WAITING: u8 = 0x06;
pub const FRAME_PING: u8 = 0x07;

pub const FRAME_FLAG_ACK: u8 = 0b10000000;
pub const FRAME_FLAG_STREAM: u8 = 0b01000000;

pub enum AckBlock {
    Block(u64),
    Gap(u8),
}

pub enum AckTimestamp {
    Delta(u8),
    FirstTimeStamp(u32),
    TimeSincePrevious(u16),
}

pub enum FrameBody {
    Padding,
    RstStream{
        error_code: u32,
        stream_id: u32,
        final_offset: u64,
    },
    ConnectionClose{
        error_code: u32,
        reason_phrase_length: u16,
        reason_phrase: Option<Vec<u8>>,
    },
    GoAway{
        error_code: u32,
        last_good_stream_id: u32,
        reason_phrase_length: u16,
        reason_phrase: Option<Vec<u8>>,
    },
    WindowUpdate{
        stream_id: u32,
        byte_offset: u64,
    },
    Blocked{
        stream_id: u32,
    },
    StopWaiting{
        least_acked_delta: u64,
    },
    Ping,
    Ack{
        num_blocks: Option<u8>,
        num_timestamps: u8,
        largest_acknowledged: u64,
        ack_delay: u16,
        ack_blocks: Vec<AckBlock>,
        timestamps: Vec<AckTimestamp>,
    },
    Stream{
        data_length: u16,
        stream_id: u32,
        offset: u64,
        stream_data: Vec<u8>,
    },
}

pub struct Frame {
    pub frame_type: u8,
    pub frame_body: FrameBody,
}
