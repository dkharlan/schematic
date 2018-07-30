#[derive(Debug)]
pub enum Error {
    FixnumParsing,
    BooleanParsing,
    UnknownToken,
    EmptyValues,
    MismatchedTypes
}
