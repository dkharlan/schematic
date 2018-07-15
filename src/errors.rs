#[derive(Debug)]
pub enum Error {
    UnimplementedParserOperation,
    FixnumParsing,
    BooleanParsing,
    UnknownToken,
    EmptyValues
}
