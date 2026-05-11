# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

import lit.Test
import lit.util
from .base import TestFormat
from dataclasses import dataclass
from typing import Optional, Tuple
import difflib
import os
import re
import shutil
import tomli


MODELS = ("refcount", "unsafe")
PTR_RE = re.compile(r"0x[0-9a-fA-F]+")

_RE_XFAIL = re.compile(r"//\s*XFAIL:\s*(.*)")
_RE_PANIC = re.compile(r"//\s*panic\s*(?::\s*(.*))?$", re.MULTILINE)
_RE_NOCOMPILE = re.compile(r"//\s*no-compile\s*(?::\s*(.*))?$", re.MULTILINE)
_RE_TRANS_FAIL = re.compile(r"//\s*translation-fail\s*(?::\s*(.*))?$", re.MULTILINE)
_RE_NONDET = re.compile(r"//\s*nondet-result\s*(?::\s*(.*))?$", re.MULTILINE)


@dataclass
class TestExpectations:
    should_panic: bool = False
    should_not_compile: bool = False
    should_not_translate: bool = False
    is_nondet_result: bool = False
    xfail: bool = False
    fail_code: int = lit.Test.FAIL

    def needs_cpp(self):
        return not (
            self.should_panic or self.is_nondet_result or self.should_not_compile
        )

    @classmethod
    def parse(cls, text, model):
        def matches(match):
            if match is None:
                return False
            models = match.group(1)
            if models is None or models.strip() == "":
                return True
            return model in re.split(r"\s*,\s*", models.strip())

        e = cls()
        xfail_m = _RE_XFAIL.search(text)
        if xfail_m:
            models = re.split(r"\s*,\s*", xfail_m.group(1))
            e.xfail = model in models
            if e.xfail:
                e.fail_code = lit.Test.XFAIL
        e.should_panic = matches(_RE_PANIC.search(text))
        e.should_not_compile = matches(_RE_NOCOMPILE.search(text))
        e.should_not_translate = matches(_RE_TRANS_FAIL.search(text))
        e.is_nondet_result = matches(_RE_NONDET.search(text))
        return e


@dataclass
class TestContext:
    cc_input: str
    is_multi: bool
    fname: str
    filepath: str
    model: str
    tmp_dir: str
    rs_file: str
    expectations: TestExpectations
    replace_expected: bool = False
    skip_run: bool = False
    build_dir: Optional[str] = None
    generated: Optional[str] = None
    pkg_name: Optional[str] = None
    cpp_bin: Optional[str] = None
    rust_bin: Optional[str] = None
    cpp_result: Optional[Tuple[str, str, int]] = None

    @classmethod
    def setup(cls, test):
        cc_input = test.getFilePath()
        is_multi = is_multi_file_test(cc_input)
        model = test.getSourcePath().split("/")[-1]
        fname = (
            os.path.basename(cc_input)
            if is_multi
            else os.path.splitext(os.path.basename(cc_input))[0]
        )
        tmp_dir = "tmp/" + fname + "-" + model

        shutil.rmtree(tmp_dir, True)
        os.makedirs(tmp_dir + "/src")

        return cls(
            cc_input=cc_input,
            is_multi=is_multi,
            fname=fname,
            filepath=cc_input if is_multi else os.path.dirname(cc_input),
            model=model,
            tmp_dir=tmp_dir,
            rs_file=tmp_dir + "/src/main.rs",
            expectations=TestExpectations.parse(load_source_text(cc_input), model),
            replace_expected=bool(os.environ.get("REPLACE_EXPECTED", False)),
            skip_run=bool(os.environ.get("SKIP_RUN", False)),
        )

    def translate(self):
        exp = self.expectations
        if self.is_multi:
            build_dir, err = setup_build_dir(self.tmp_dir, self.cc_input)
            if err is not None:
                return (exp.fail_code, err)
            self.build_dir = build_dir

        cmd = cpp2rust_command(self.cc_input, self.build_dir, self.model, self.rs_file)
        out, err, returncode = lit.util.executeCommand(cmd)

        if not os.path.exists(self.rs_file):
            return (
                exp.fail_code,
                "no out file (rc="
                + str(returncode)
                + ")\n"
                + "cmd: "
                + " ".join(cmd)
                + "\n"
                + "\nstderr: "
                + err
                + "\nstdout: "
                + out,
            )

        with open(self.rs_file, "r") as f:
            self.generated = f.read()

        if returncode != 0:
            if exp.should_not_translate:
                return (lit.Test.XFAIL, "")
            return (exp.fail_code, "cpp2rust failed\n" + err)

        if exp.should_not_translate:
            return (exp.fail_code, "expected translation-fail but cpp2rust succeeded")
        return None

    def check_expected(self):
        exp = self.expectations
        if exp.should_not_compile:
            return None

        expected_file = get_expected_file(self.filepath, self.model, self.fname)
        if not os.path.exists(expected_file) and not self.replace_expected:
            return (exp.fail_code, "no expected file")

        if self.replace_expected:
            update_expected(self.generated, expected_file)

        with open(expected_file, "r") as f:
            expected = f.read()

        if self.generated != expected:
            diff = "".join(
                difflib.unified_diff(
                    expected.splitlines(keepends=True),
                    self.generated.splitlines(keepends=True),
                    fromfile="expected",
                    tofile="generated",
                )
            )
            return (exp.fail_code, "different output\n" + diff)
        return None

    def build_cpp(self):
        exp = self.expectations
        if self.skip_run or not exp.needs_cpp():
            return None

        if self.build_dir is not None:
            cmd = ["cmake", "--build", self.build_dir]
            _, _, rc = lit.util.executeCommand(cmd)
            if rc != 0:
                return (exp.fail_code, "cmake build failed")
            self.cpp_bin = os.path.join(self.build_dir, "app")
            return None

        cc = (
            os.environ.get("CC", "clang")
            if self.cc_input.endswith(".c")
            else os.environ.get("CXX", "clang++")
        )
        self.cpp_bin = self.tmp_dir + "/cpp"
        cmd = [cc, "-O3", "-o", self.cpp_bin, self.cc_input]
        _, _, rc = lit.util.executeCommand(cmd)
        if rc != 0:
            return (exp.fail_code, cc + " failed")
        return None

    def build_rust(self):
        exp = self.expectations
        rust_version = read_rust_version()
        self.pkg_name = "test_" + re.sub(
            r"[^a-zA-Z0-9_]", "_", os.path.basename(self.tmp_dir)
        )

        with open(self.tmp_dir + "/rust-toolchain.toml", "w") as f:
            f.write(f'[toolchain]\nchannel = "{rust_version}"\n')
        with open(self.tmp_dir + "/Cargo.toml", "w") as f:
            f.write(f"""
[package]
name = "{self.pkg_name}"
version = "0.1.0"
edition = "2021"
rust-version = "{rust_version}"

[[bin]]
name = "{self.pkg_name}"
path = "src/main.rs"

[dependencies]
libc = "0.2.169"
libcc2rs = {{ path = "../../../libcc2rs" }}
""")

        cmd = ["cargo", "build", "--release", "--quiet"]
        _, err, returncode = lit.util.executeCommand(cmd, self.tmp_dir, env=cargo_env())
        if exp.should_not_compile:
            if returncode != 0:
                return (lit.Test.XFAIL, "")
            return (exp.fail_code, "expected no-compile but compiled successfully")
        if returncode != 0:
            return (exp.fail_code, "cargo failed\n" + err)

        self.rust_bin = os.path.join(shared_target_dir(), "release", self.pkg_name)
        return None

    def run_cpp(self):
        exp = self.expectations
        if self.skip_run or not exp.needs_cpp():
            return None
        out, err, rc = lit.util.executeCommand(self.cpp_bin)
        self.cpp_result = (out, err, rc)
        return None

    def run_rust(self):
        exp = self.expectations
        if self.skip_run:
            return None

        if exp.should_panic:
            _, err, rc = lit.util.executeCommand(self.rust_bin)
            err = str(err)
            if not re.search(r"thread 'main' \(\d+\) panicked at", err) or rc != 101:
                return (exp.fail_code, "expected panic\n" + err)
            return None

        if exp.is_nondet_result:
            lit.util.executeCommand(self.rust_bin)
            return None

        out_rs, err_rs, rc_rs = lit.util.executeCommand(self.rust_bin)
        out_cpp, err_cpp, rc_cpp = self.cpp_result
        out_cpp_cmp = PTR_RE.sub("0xPTR", out_cpp)
        out_rs_cmp = PTR_RE.sub("0xPTR", out_rs)
        if out_cpp_cmp != out_rs_cmp or rc_cpp != rc_rs or err_rs != err_cpp:
            return (
                exp.fail_code,
                "different output\n" + out_cpp + err_cpp + out_rs + err_rs,
            )
        return None

    def finalize(self, result):
        shutil.rmtree(self.tmp_dir, True)
        return result


class Cpp2RustTest(TestFormat):
    def __init__(self):
        os.environ["RUSTFLAGS"] = "-Awarnings"

    def execute(self, test, litConfig):
        ctx = TestContext.setup(test)

        result = (
            ctx.translate()
            or ctx.check_expected()
            or ctx.build_cpp()
            or ctx.build_rust()
            or ctx.run_cpp()
            or ctx.run_rust()
        )
        if result is not None:
            return ctx.finalize(result)

        if ctx.expectations.xfail:
            return ctx.finalize((lit.Test.FAIL, "did not fail as expected"))
        return ctx.finalize((lit.Test.PASS, ""))

    def getTestsForPath(self, testSuite, path_in_suite, litConfig, localConfig):
        source_path = testSuite.getSourcePath(path_in_suite)
        for model in MODELS:
            yield lit.Test.Test(
                testSuite, path_in_suite + (model,), localConfig, source_path
            )

    def getTestsInDirectory(self, testSuite, path_in_suite, litConfig, localConfig):
        source_path = testSuite.getSourcePath(path_in_suite)
        if is_multi_file_test(source_path):
            for t in self.getTestsForPath(
                testSuite, path_in_suite, litConfig, localConfig
            ):
                yield t
            return
        for entry in os.listdir(source_path):
            full = os.path.join(source_path, entry)
            if os.path.isfile(full) and (
                entry.endswith(".cpp") or entry.endswith(".c")
            ):
                for t in self.getTestsForPath(
                    testSuite, path_in_suite + (entry,), litConfig, localConfig
                ):
                    yield t


def read_rust_version():
    toolchain_path = os.path.join(
        os.path.dirname(__file__), "../../../../libcc2rs/rust-toolchain.toml"
    )
    with open(toolchain_path, "rb") as f:
        return tomli.load(f)["toolchain"]["channel"]


def shared_target_dir():
    return os.path.abspath(
        os.path.join(os.path.dirname(__file__), "../../../../build/tmp/cargo-target")
    )


def cargo_env():
    return dict(os.environ, CARGO_TARGET_DIR=os.path.abspath(shared_target_dir()))


def is_multi_file_test(path):
    return os.path.isdir(path) and os.path.exists(os.path.join(path, "CMakeLists.txt"))


def load_source_text(cc_input):
    if os.path.isdir(cc_input):
        expectations_path = os.path.join(cc_input, "test.expectations")
        if not os.path.exists(expectations_path):
            return ""
        lines = []
        with open(expectations_path, "r") as f:
            for line in f:
                s = line.strip()
                if not s or s.startswith("#"):
                    continue
                lines.append("// " + s)
        return "\n".join(lines)
    with open(cc_input, "r") as f:
        return f.read()


def setup_build_dir(tmp_dir, cc_input):
    build_dir = os.path.abspath(os.path.join(tmp_dir, "cmake-build"))
    os.makedirs(build_dir, exist_ok=True)
    cmd = [
        "cmake",
        "-S",
        cc_input,
        "-B",
        build_dir,
        "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
    ]
    _, err, rc = lit.util.executeCommand(cmd)
    if rc != 0:
        return None, "cmake configure failed\n" + err
    return build_dir, None


def cpp2rust_command(cc_input, build_dir, model, rs_file):
    if build_dir is not None:
        return [
            "./cpp2rust/cpp2rust",
            "-dir",
            build_dir,
            "-model",
            model,
            "-o",
            rs_file,
        ]
    return ["./cpp2rust/cpp2rust", "-file", cc_input, "-model", model, "-o", rs_file]


def get_expected_file(filepath, model, fname):
    return filepath + "/out/" + model + "/" + fname + ".rs"


def update_expected(generated, expected_path):
    os.makedirs(os.path.dirname(expected_path), exist_ok=True)
    with open(expected_path, "w") as f:
        f.write(generated)
