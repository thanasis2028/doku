#[derive(Doku)]
pub struct Ty {
    /// Some comment
    f1: Option<String>,

    /// Another comment
    #[doku(example = "bar-value")]
    f2: Option<NestedString>,

    /// Yet another comment
    #[doku(example = "zar-value")]
    zar: Option<Option<Option<String>>>,
}

#[derive(Doku)]
struct NestedString(String);
