use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("bbshark").unwrap();
    cmd
        .assert()
        .success()
        .stdout("Baby shark... doo, doo, doo, doo, doo, doo.\nBaby shark!");
}

#[test]
fn runs_with_d_flag() {
    let mut cmd = Command::cargo_bin("bbshark").unwrap();
    cmd
        .arg("-d")
        .arg("3")
        .assert()
        .success()
        .stdout("Baby shark... doo, doo, doo.\nBaby shark!");
}


#[test]
fn fails_with_d_flag_if_not_numeric() {
    let mut cmd = Command::cargo_bin("bbshark").unwrap();
    cmd
    .arg("-d")
    .arg("abc")
    .assert()
    .failure()
    .stderr("Must enter an unsigned integer between 1 and 127 (u8). Got 'abc'\n");
}

#[test]
fn fails_with_d_flag_if_zero() {
    let mut cmd = Command::cargo_bin("bbshark").unwrap();
    cmd
    .arg("-d")
    .arg("0")
    .assert()
    .failure()
    .stderr("Must enter an unsigned integer between 1 and 127 (u8). Got '0'\n");
}