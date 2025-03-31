// node is the struct implements Node interface except for Accept method.
// Node implementations should embed it in.
pub struct Node  {
    pub utf8_text: String,
    pub enc:      charset.Encoding
    once     *sync.Once

    text   string
    offset int
}