use assert_cmd::Command;

const NAME: &str = "catr";

#[test]
fn test_no_lines()  {
    let mut cmd = Command::cargo_bin(NAME).unwrap();
    cmd
        .arg("-")
        .write_stdin("aaa\nbbb")
        .assert()
        .stdout("aaa\nbbb\n");
}

#[test]
fn test_lines()  {
    let mut cmd = Command::cargo_bin(NAME).unwrap();
    cmd
        .arg("-n")
        .arg("-")
        .write_stdin("aaa\nbbb")
        .assert()
        .stdout(format!("{:>6}\t{}\n{:>6}\t{}\n", "1", "aaa", "2", "bbb"));
}

#[test]
fn test_file() {
    let mut cmd = Command::cargo_bin(NAME).unwrap();
    cmd
        .arg("tests/file1.txt")
        .assert()
        .stdout("aaa\nbbb\nccc\n");
}