use super::*;

// list metadata type
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Meta {
    head: ListIndex,
    len: u64,
}

pub const META_SIZE: usize = INDEX_BYTES + 8;

impl Meta {
    pub fn encode(self) -> Record {
        let mut out = [0u8; META_SIZE];
        out[..INDEX_BYTES].copy_from_slice(&self.head.to_be_bytes());
        out[INDEX_BYTES..].copy_from_slice(&self.len.to_be_bytes());

        Record::FromData(Tag::List, (&out).into())
    }

    pub fn decode(inp: &Record) -> Result<Self, Error> {
        if inp.tag() != Tag::List {
            Err(Error::BadType(Tag::Table, inp.tag()))?
        } else if inp.len() != META_SIZE {
            Err(ListError::InvalidMeta(inp.data()))?
        } else {
            let mut head_buf = [0u8; INDEX_BYTES];
            head_buf.copy_from_slice(&inp[..INDEX_BYTES]);
            let mut len_buf = [0u8; 8];
            len_buf.copy_from_slice(&inp[INDEX_BYTES..]);
            Ok(Self {
                head: ListIndex::from_be_bytes(head_buf),
                len: u64::from_be_bytes(len_buf),
            })
        }
    }

    pub fn mk_key(&self, ix: i64) -> Option<ListIndex> {
        let offset = ix.rem_euclid(self.len as i64);
        let valid_ix = ix <= offset; // ix <= offset <-> (ix < 0 /\ ix.abs() <= self.len) \/ (ix > 0 /\ ix < self.len)
        if valid_ix {
            Some(self.head + offset as ListIndex)
        } else {
            None
        }
    }

    pub fn head_ix(&self) -> Option<ListIndex> {
        if self.len != 0 {
            Some(self.head)
        } else {
            None
        }
    }

    pub fn tail_ix(&self) -> Option<ListIndex> {
        if self.len != 0 {
            Some(self.head + self.len as ListIndex - 1)
        } else {
            None
        }
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(super) fn push_front(&mut self) -> ListIndex {
        self.head -= 1;
        self.len += 1;

        self.head
    }

    pub(super) fn pop_front(&mut self) -> Option<ListIndex> {
        let res = self.head_ix()?;
        self.head += 1;
        self.len -= 1;
        Some(res)
    }

    pub(super) fn push_back(&mut self) -> ListIndex {
        self.len += 1;
        self.head + self.len as ListIndex - 1
    }

    pub(super) fn pop_back(&mut self) -> Option<ListIndex> {
        let res = self.tail_ix()?;
        self.len -= 1;
        Some(res)
    }
}
