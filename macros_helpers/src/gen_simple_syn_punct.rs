#[derive(Debug, Clone, Copy)]
pub struct SimpleSynPunctuated;

#[must_use]
pub const fn gen_simple_syn_punct<Value>(_value: &Value) -> SimpleSynPunctuated
where
    Value: ?Sized,
{
    SimpleSynPunctuated
}

#[must_use]
pub const fn string_syn_punct() -> SimpleSynPunctuated {
    SimpleSynPunctuated
}
