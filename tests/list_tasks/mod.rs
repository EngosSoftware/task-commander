#[test]
fn _0001() {
    cli_assert::command!()
        .success()
        .code(0)
        .stdout(
            r"Available tasks
  build             Builds in debug mode
  clean             Cleans all targets
  clippy            Runs clippy for all targets
  cov               Generates code coverage report in text format
  cov-badge         Generates the detailed code coverage badge
  cov-html          Generates code coverage report in HTML format
  cov-html-open     Generates code coverage report in HTML format
  doc               Generates documentation
  doc-open          Generates documentation and opens in browser
  doc-private       Generates documentation
  doc-private-open  Generates documentation and opens in browser
  fmt
  install           Builds and installs release version from local sources
  manual            Builds the manual from its markdown files
  readme            Recreates README file
  serve             Serves the manual locally and rebuilds on changes
  test              Runs tests in debug mode
  testn             Runs tests in debug mode
  uninstall         Uninstalls previously installed local version
",
        )
        .stderr("")
        .execute();
}
