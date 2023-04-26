pub type DocumentServiceResult<T> = Result<T, DocumentServiceError>;

#[derive(Debug)]
pub enum DocumentServiceError {
    IntegerOverflow,
    SegmentTooLong,
    DataOverCapacity(usize, usize),
}
