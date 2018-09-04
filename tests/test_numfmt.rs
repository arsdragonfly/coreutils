use common::util::*;

#[test]
fn test_from_si() {
    new_ucmd!()
        .args(&["--from=si"])
        .pipe_in("1000\n1.1M\n0.1G")
        .run()
        .stdout_is("1000\n1100000\n100000000");
}

#[test]
fn test_from_iec() {
    new_ucmd!()
        .args(&["--from=iec"])
        .pipe_in("1024\n1.1M\n0.1G")
        .run()
        .stdout_is("1024\n1153434\n107374183");
}

#[test]
fn test_from_iec_i() {
    new_ucmd!()
        .args(&["--from=iec-i"])
        .pipe_in("1024\n1.1Mi\n0.1Gi")
        .run()
        .stdout_is("1024\n1153434\n107374183");
}

#[test]
fn test_from_auto() {
    new_ucmd!()
        .args(&["--from=auto"])
        .pipe_in("1K\n1Ki")
        .run()
        .stdout_is("1000\n1024");
}

#[test]
fn test_to_si() {
    new_ucmd!()
        .args(&["--to=si"])
        .pipe_in("1000\n1100000\n100000000")
        .run()
        .stdout_is("1.0K\n1.1M\n100.0M");
    /*
        TODO: figure out if we should stick with GNU numfmt's formatting:
        .stdout_is("1.0K\n1.1M\n100M");
        */
}

#[test]
fn test_to_iec() {
    new_ucmd!()
        .args(&["--to=iec"])
        .pipe_in("1024\n1153434\n107374182")
        .run()
        .stdout_is("1.0K\n1.2M\n102.4M");
    /*
        GNU numfmt:
        .stdout_is("1.0K\n1.2M\n103M");
        */
}

#[test]
fn test_to_iec_i() {
    new_ucmd!()
        .args(&["--to=iec-i"])
        .pipe_in("1024\n1153434\n107374182")
        .run()
        .stdout_is("1.0Ki\n1.2Mi\n102.4Mi");
    /*
        GNU numfmt:
        .stdout_is("1.0Ki\n1.2Mi\n103Mi");
        */
}

#[test]
fn test_input_from_free_arguments() {
    new_ucmd!()
        .args(&["--from=si", "1K", "1.1M", "0.1G"])
        .run()
        .stdout_is("1000\n1100000\n100000000");
}

#[test]
fn test_padding() {
    new_ucmd!()
        .args(&["--from=si", "--padding=8"])
        .pipe_in("1K\n1.1M\n0.1G")
        .run()
        .stdout_is("    1000\n 1100000\n100000000");
}

#[test]
fn test_negative_padding() {
    new_ucmd!()
        .args(&["--from=si", "--padding=-8"])
        .pipe_in("1K\n1.1M\n0.1G")
        .run()
        .stdout_is("1000    \n1100000 \n100000000");
}

#[test]
fn test_header() {
    new_ucmd!()
        .args(&["--from=si", "--header=2"])
        .pipe_in("header\nheader2\n1K\n1.1M\n0.1G")
        .run()
        .stdout_is("header\nheader2\n1000\n1100000\n100000000");
}

#[test]
fn test_header_default() {
    new_ucmd!()
        .args(&["--from=si", "--header"])
        .pipe_in("header\n1K\n1.1M\n0.1G")
        .run()
        .stdout_is("header\n1000\n1100000\n100000000");
}

//numbers for the rounding test
const TEST_NUMBERS: &'static str =
    "-9999.9\n-999.9\n-99.9\n-9.9\n-0.9\n0.9\n9.9\n99.9\n999.9\n9999.9";

#[test]
fn test_rounding_from_zero() {
    //This is the default
    new_ucmd!()
        .args(&["--to=si"])
        .pipe_in(TEST_NUMBERS)
        .run()
        .stdout_is("-10.0K\n-1.0K\n-100\n-10\n-1\n1\n10\n100\n1.0K\n10.0K");
    /*
        GNU numfmt:
        .stdout_is("-10K\n-1.0K\n-100\n-10\n-1\n1\n10\n100\n1.0K\n10K");
        */
}

#[test]
fn test_rounding_towards_zero() {
    new_ucmd!()
        .args(&["--to=si", "--round=towards-zero"])
        .pipe_in(TEST_NUMBERS)
        .run()
        .stdout_is("-9.9K\n-999\n-99\n-9\n-0\n0\n9\n99\n999\n9.9K");
    /*
        GNU numfmt (things start to get inconsistent):
        .stdout_is("-9.9K\n-999\n-99\n-10\n-1\n1\n10\n99\n999\n9.9K");
        */
}

#[test]
fn test_rounding_up() {
    new_ucmd!()
        .args(&["--to=si", "--round=up"])
        .pipe_in(TEST_NUMBERS)
        .run()
        .stdout_is("-9.9K\n-999\n-99\n-9\n-0\n1\n10\n100\n1.0K\n10.0K");
    /*
        GNU numfmt (yes, -9.9 rounds up to -10):
        .stdout_is("-9.9K\n-999\n-99\n-10\n-1\n1\n10\n100\n1.0K\n10K");
        */
}

#[test]
fn test_rounding_down() {
    new_ucmd!()
        .args(&["--to=si", "--round=down"])
        .pipe_in(TEST_NUMBERS)
        .run()
        .stdout_is("-10.0K\n-1.0K\n-100\n-10\n-1\n0\n9\n99\n999\n9.9K\n");
    /*
        GNU numfmt (yes, 9.9 rounds down to 10):
        .stdout_is("-10K\n-1.0K\n-100\n-10\n-1\n1\n10\n99\n999\n9.9K\n");
        */
}

#[test]
fn test_rounding_nearest() {
    new_ucmd!()
        .args(&["--to=si", "--round=nearest"])
        .pipe_in(TEST_NUMBERS)
        .run()
        .stdout_is("-10.0K\n-1.0K\n-100\n-10\n-1\n1\n10\n100\n1.0K\n10.0K");
    //an interesting test
    new_ucmd!()
        .args(&["--to=si", "--round=nearest"])
        .pipe_in("8.46\n9.46\n10.46")
        .run()
        .stdout_is("8\n9\n10");
    /*
        GNU numfmt (if you really want the conformity):
        .stdout_is("8\n10\n10");
        */
}
