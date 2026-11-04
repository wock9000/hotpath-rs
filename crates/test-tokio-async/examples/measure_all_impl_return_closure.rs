struct Worktree(String);

#[hotpath::measure_all]
impl Worktree {
    fn label<'a>(&'a self) -> impl Fn() -> &'a str + 'a {
        || self.0.as_str()
    }
}

impl Worktree {
    // The explicit lifetimes are the point: this example is the regression
    // case for the macro's lifetime handling (61d7e57), so keep them.
    #[allow(clippy::needless_lifetimes)]
    #[hotpath::measure(log = true)]
    fn label_logged<'a>(&'a self) -> &'a str {
        self.0.as_str()
    }
}

fn main() {
    let worktree = Worktree("local-worktree".to_string());
    let _ = worktree.label()();
    let _ = worktree.label_logged();
}
