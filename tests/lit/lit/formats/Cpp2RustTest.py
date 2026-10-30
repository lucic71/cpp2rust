# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

import lit.Test
import lit.util
from lit.BooleanExpression import BooleanExpression
from .._buildconfig import BUILD_DIR, CXX, CC
from .base import TestFormat
from dataclasses import dataclass
from pathlib import Path
from typing import NamedTuple, Optional
import difflib
import os
import re
import shlex
import shutil


MODELS = ("refcount", "unsafe")
IGNORED_SUFFIXES = (".verify.cpp", ".sh.cpp", ".compile.fail.cpp", ".compile.pass.cpp")
PTR_RE = re.compile(r"0x[0-9a-fA-F]+")

RE_XFAIL = re.compile(r"//\s*XFAIL:\s*(.*)")
RE_PANIC_UB = re.compile(r"//\s*panic-ub\s*(?::\s*(.*))?$", re.MULTILINE)
RE_PANIC = re.compile(r"//\s*panic\s*(?::\s*(.*))?$", re.MULTILINE)
RE_NOCOMPILE = re.compile(r"//\s*no-compile\s*(?::\s*(.*))?$", re.MULTILINE)
RE_TRANS_FAIL = re.compile(r"//\s*translation-fail\s*(?::\s*(.*))?$", re.MULTILINE)
RE_NONDET = re.compile(r"//\s*nondet-result\s*(?::\s*(.*))?$", re.MULTILINE)
RE_ADDITIONAL_COMPILE_FLAGS = re.compile(
    r"//\s*ADDITIONAL_COMPILE_FLAGS(?:\(([^)]*)\))?:\s*(.*)$", re.MULTILINE
)
RE_FILE_DEPENDENCIES = re.compile(r"//\s*FILE_DEPENDENCIES:\s*(.*)$", re.MULTILINE)
RE_REQUIRES = re.compile(r"//\s*REQUIRES:\s*(.*)$", re.MULTILINE)
RE_UNSUPPORTED = re.compile(r"//\s*UNSUPPORTED:\s*(.*)$", re.MULTILINE)
RE_DG_DO = re.compile(r"\{\s*dg-do\s+(\w+)\s*(.*?)\s*\}\s*$", re.MULTILINE)
RE_DG_TARGET_STD = re.compile(r"\{\s*target\s+c\+\+(\d+)\s*\}")
RE_DG_SKIP = re.compile(
    r"\{\s*(dg-require-[\w-]+|dg-final|dg-output|dg-additional-files|dg-skip-if|dg-xfail-run-if)\b"
)
RE_DG_OPTIONS = re.compile(
    r'\{\s*dg-(?:additional-)?options\s+"([^"]*)"\s*'
    r"(?:\{\s*target\s+([^}]*?)\s*\}\s*)?\}"
)
# Target selectors of dg-options that can be decided for the host
DG_TARGET_SELECTORS = {"le": True, "be": False}
RE_INCLUDE = re.compile(r'^\s*#\s*include\s*[<"]([^>"]+)[>"]', re.MULTILINE)
RE_RUSTC_ERROR = re.compile(r"^(error(?:\[E\d+\])?: .*)$", re.MULTILINE)
TOLERATED_RUSTC_ERRORS = (
    re.compile(r"error\[E0782\]: "),
    re.compile(
        r"error\[E0599\]: no (?:associated function or constant|method) named "
        r"`[^`]+` found for struct `std_[A-Za-z_]+` in the current scope$"
    ),
    re.compile(
        r"error\[E0433\]: cannot find module or crate `std_[A-Za-z_]+Impl` "
        r"in this scope$"
    ),
    re.compile(
        r"error\[E0560\]: struct `[^`]+` has no field named "
        r"`(?:__|_[A-Z])[^`]*`$"
    ),
)


@dataclass
class TestExpectations:
    should_panic: bool = False
    should_panic_ub: bool = False
    should_not_compile: bool = False
    should_not_translate: bool = False
    is_nondet_result: bool = False
    xfail: bool = False
    fail_code: int = lit.Test.FAIL

    @classmethod
    def parse(cls, text, model, features=()):
        def matches(match):
            if match is None:
                return False
            models = match.group(1)
            if models is None or models.strip() == "":
                return True
            return model in re.split(r"\s*,\s*", models.strip())

        xfail = False
        for xfail_m in RE_XFAIL.finditer(text):
            for item in re.split(r"\s*,\s*", xfail_m.group(1).strip()):
                if item == model or item == "*":
                    xfail = True
                elif item and item not in MODELS:
                    xfail = xfail or BooleanExpression.evaluate(item, features)
        return cls(
            should_panic=matches(RE_PANIC.search(text)),
            should_panic_ub=matches(RE_PANIC_UB.search(text)),
            should_not_compile=matches(RE_NOCOMPILE.search(text)),
            should_not_translate=matches(RE_TRANS_FAIL.search(text)),
            is_nondet_result=matches(RE_NONDET.search(text)),
            xfail=xfail,
            fail_code=lit.Test.XFAIL if xfail else lit.Test.FAIL,
        )


class RunResult(NamedTuple):
    stdout: str
    stderr: str
    returncode: int


@dataclass
class TestContext:
    cc_input: Path
    is_multi: bool
    fname: str
    filepath: Path
    model: str
    tmp_dir: Path
    rs_file: Path
    expectations: TestExpectations
    extra_cxxflags: list = None
    force_std: Optional[str] = None
    support_sources: list = None
    replace_expected: bool = False
    skip_run: bool = False
    build_dir: Optional[Path] = None
    generated: Optional[str] = None
    cpp_bin: Optional[Path] = None
    rust_bin: Optional[Path] = None
    cpp_result: Optional[RunResult] = None
    rust_result: Optional[RunResult] = None
    stdin_text: Optional[str] = None

    @classmethod
    def setup(cls, test):
        cc_input = Path(test.getFilePath())
        is_multi = is_multi_file_test(cc_input)
        model = test.getSourcePath().split("/")[-1]
        rel = cc_input.relative_to(test.suite.source_root)
        fname = cc_input.name if is_multi else cc_input.stem
        tmp_name = str(rel if is_multi else rel.with_suffix("")).replace("/", "_")
        tmp_dir = get_temp_dir() / f"{tmp_name}-{model}"

        shutil.rmtree(tmp_dir, ignore_errors=True)
        tmp_dir.mkdir(parents=True)
        source_text = load_source_text(cc_input)
        for dep in parse_feature_list(RE_FILE_DEPENDENCIES, source_text):
            shutil.copy2(cc_input.parent / dep, tmp_dir)
        extra_cxxflags = list(getattr(test.config, "extra_cxxflags", []))
        if default_std := getattr(test.config, "dg_default_std", None):
            extra_cxxflags.append(f"-std=gnu++{dg_std(source_text, default_std)}")
        test_cxxflags = parse_additional_compile_flags(
            source_text, test.config.available_features
        )
        force_std = getattr(test.config, "force_std", None)
        if force_std and cc_input.suffix != ".c":
            test_cxxflags = [f for f in test_cxxflags if not f.startswith("-std=")]
            test_cxxflags.append(f"-std=gnu++{force_std}")
        extra_cxxflags += test_cxxflags

        return cls(
            cc_input=cc_input,
            is_multi=is_multi,
            fname=fname,
            filepath=cc_input if is_multi else cc_input.parent,
            model=model,
            tmp_dir=tmp_dir,
            rs_file=tmp_dir / "main.rs",
            expectations=TestExpectations.parse(
                source_text, model, test.config.available_features
            ),
            extra_cxxflags=extra_cxxflags,
            force_std=force_std,
            support_sources=collect_support_sources(cc_input, test.config),
            replace_expected=bool(os.environ.get("REPLACE_EXPECTED", False)),
            stdin_text=read_stdin_file(cc_input, is_multi),
            skip_run=bool(os.environ.get("SKIP_RUN", False)),
        )

    def translate(self):
        exp = self.expectations
        if self.is_multi:
            build_dir, err = setup_build_dir(self.tmp_dir, self.cc_input, self.force_std)
            if err is not None:
                return (exp.fail_code, err)
            self.build_dir = build_dir
        elif self.support_sources:
            build_dir, err = setup_build_dir(
                self.tmp_dir, self.write_support_project(), self.force_std
            )
            if err is not None:
                return (exp.fail_code, err)
            self.build_dir = build_dir

        cmd = cpp2rust_command(
            self.cc_input,
            self.build_dir,
            self.model,
            self.rs_file,
            self.extra_cxxflags,
        )
        out, err, returncode = lit.util.executeCommand(cmd)

        if returncode != 0 and exp.should_not_translate:
            return (lit.Test.XFAIL, "")

        if not self.rs_file.exists():
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

        if returncode != 0:
            if exp.should_not_translate:
                return (lit.Test.XFAIL, "")
            return (exp.fail_code, "cpp2rust failed\n" + err)

        if exp.should_not_translate:
            return (exp.fail_code, "expected translation-fail but cpp2rust succeeded")

        self.generated = self.rs_file.read_text()
        return None

    def write_support_project(self):
        src_dir = self.tmp_dir / "cmake-src"
        src_dir.mkdir()
        sources = [self.cc_input] + self.support_sources
        flags = [f"-I{self.cc_input.parent}"] + self.extra_cxxflags
        (src_dir / "CMakeLists.txt").write_text(
            "cmake_minimum_required(VERSION 3.16)\n"
            "project(app LANGUAGES CXX)\n"
            f"add_executable(app {cmake_list(sources)})\n"
            f"target_compile_options(app PRIVATE {cmake_list(flags)})\n"
            f"target_link_options(app PRIVATE {cmake_list(self.extra_cxxflags)} -latomic)\n"
            + "".join(
                f'set_source_files_properties("{src}" PROPERTIES COMPILE_OPTIONS '
                f'"-Wno-builtin-macro-redefined;-D__FILE__=\\"{src.name}\\"")\n'
                for src in sources
            )
        )
        return src_dir

    def check_expected(self):
        exp = self.expectations
        # We don't care if no-compile tests have a corresponding generated file.
        if exp.should_not_compile:
            return None

        expected_file = get_expected_file(self.filepath, self.model, self.fname)
        if not expected_file.exists() and not self.replace_expected:
            return (exp.fail_code, "no expected file")

        if self.replace_expected:
            update_expected(self.generated, expected_file)

        expected = expected_file.read_text()

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
        if self.build_dir is not None:
            cmd = ["cmake", "--build", str(self.build_dir)]
            out, err, rc = lit.util.executeCommand(cmd)
            if rc != 0:
                return (exp.fail_code, "cmake build failed\n" + out + err)
            self.cpp_bin = self.build_dir / "app"
            return None

        cc = CC if self.cc_input.suffix == ".c" else CXX
        self.cpp_bin = self.tmp_dir / "cpp"
        cmd = [
            cc,
            "-O3",
            "-o",
            str(self.cpp_bin),
            str(self.cc_input),
            "-Wno-builtin-macro-redefined",
            '-D__FILE__="' + os.path.basename(self.cc_input) + '"',
        ]
        cmd.extend(self.extra_cxxflags)
        cmd.append("-latomic")
        _, err, rc = lit.util.executeCommand(cmd)
        if rc != 0:
            return (exp.fail_code, str(cc) + " failed\n" + err)
        return None

    def build_rust(self):
        exp = self.expectations

        cc2rs_dir = BUILD_DIR / "libcc2rs-target" / "release"
        libc_dep_deps = BUILD_DIR / "libc-dep-target" / "release" / "deps"
        # pick the most recently compiled libc
        libc_rlib = max(
            libc_dep_deps.glob("liblibc-*.rlib"),
            key=lambda p: p.stat().st_mtime,
        )
        nix_rlib = max(
            libc_dep_deps.glob("libnix-*.rlib"),
            key=lambda p: p.stat().st_mtime,
        )
        jiff_rlib = max(
            libc_dep_deps.glob("libjiff-*.rlib"),
            key=lambda p: p.stat().st_mtime,
        )
        cmd = [
            "rustc",
            "+" + read_rust_version(),
            "--edition",
            "2024",
            "main.rs",
            "-A",
            "warnings",
            "-C",
            "opt-level=3",  # Equivalent to --release
            "-C",
            "strip=symbols",
            "-C",
            "panic=abort",
            "--out-dir",
            str(self.tmp_dir),
            "-L",
            f"dependency={cc2rs_dir / 'deps'}",
            "-L",
            f"dependency={libc_dep_deps}",
            "--extern",
            f"libcc2rs={cc2rs_dir / 'liblibcc2rs.rlib'}",
            "--extern",
            f"libc={libc_rlib}",
            "--extern",
            f"nix={nix_rlib}",
            "--extern",
            f"jiff={jiff_rlib}",
        ]
        _, err, returncode = lit.util.executeCommand(cmd, str(self.tmp_dir))
        if exp.should_not_compile:
            if returncode != 0:
                return (lit.Test.XFAIL, "")
            return (exp.fail_code, "expected no-compile but compiled successfully")
        if returncode != 0:
            if only_tolerated_errors(err):
                return self.success_result()
            return (exp.fail_code, "rustc failed\n" + err)

        self.rust_bin = self.tmp_dir / "main"
        return None

    def run_cpp(self):
        if self.skip_run:
            return None
        self.cpp_result = RunResult(
            *lit.util.executeCommand(
                str(self.cpp_bin), str(self.tmp_dir), input=self.stdin_text
            )
        )

        if (
            not self.expectations.should_panic_ub
            and not self.expectations.is_nondet_result
            and self.cpp_result.returncode != 0
        ):
            return (
                self.expectations.fail_code,
                "C++ program returned nonzero exit code "
                + str(self.cpp_result.returncode)
                + ", non-ub tests must return 0",
            )
        return None

    def run_rust(self):
        exp = self.expectations
        if self.skip_run:
            return None
        self.rust_result = RunResult(
            *lit.util.executeCommand(
                str(self.rust_bin), str(self.tmp_dir), input=self.stdin_text
            )
        )

        if exp.should_panic_ub:
            err = str(self.rust_result.stderr)
            if not re.search(
                r"thread 'main' \(\d+\) panicked at", err
            ) or self.rust_result.returncode not in [-6, 101]:
                return (exp.fail_code, "expected panic\n" + err)
            return self.success_result()

        if exp.should_panic:
            err = str(self.rust_result.stderr)
            if not re.search(
                r"thread 'main' \(\d+\) panicked at", err
            ) or self.rust_result.returncode not in [-6, 101]:
                return (exp.fail_code, "expected panic\n" + err)
            return (lit.Test.XFAIL, "")

        if re.search(
            r"thread 'main' \(\d+\) panicked at", str(self.rust_result.stderr)
        ) and self.rust_result.returncode in [-6, 101]:
            return self.success_result()

        if exp.is_nondet_result:
            return self.success_result()
        return None

    def compare(self):
        exp = self.expectations
        if self.skip_run:
            return None

        cpp = self.cpp_result
        rs = self.rust_result
        out_cpp_cmp = PTR_RE.sub("0xPTR", cpp.stdout)
        out_rs_cmp = PTR_RE.sub("0xPTR", rs.stdout)
        if (
            out_cpp_cmp != out_rs_cmp
            or cpp.returncode != rs.returncode
            or cpp.stderr != rs.stderr
        ):
            return (
                exp.fail_code,
                "different output\n" + cpp.stdout + cpp.stderr + rs.stdout + rs.stderr,
            )
        return None

    def success_result(self):
        if self.expectations.xfail:
            return (lit.Test.FAIL, "did not fail as expected")
        return (lit.Test.PASS, "")

    def finalize(self, result):
        shutil.rmtree(self.tmp_dir, ignore_errors=True)
        return result


class Cpp2RustTest(TestFormat):
    def __init__(self):
        os.environ["RUSTFLAGS"] = "-Awarnings"

    def execute(self, test, litConfig):
        if unsupported := unsupported_reason(test):
            return lit.Test.Result(lit.Test.UNSUPPORTED, unsupported)
        ctx = TestContext.setup(test)
        result = (
            ctx.translate()
            or ctx.check_expected()
            or ctx.build_rust()
            or ctx.build_cpp()
            or ctx.run_cpp()
            or ctx.run_rust()
            or ctx.compare()
            or ctx.success_result()
        )
        return ctx.finalize(result)

    def getTestsForPath(self, testSuite, path_in_suite, litConfig, localConfig):
        source_path = testSuite.getSourcePath(path_in_suite)
        for model in MODELS:
            yield lit.Test.Test(
                testSuite, path_in_suite + (model,), localConfig, source_path
            )

    def getTestsInDirectory(self, testSuite, path_in_suite, litConfig, localConfig):
        source_path = Path(testSuite.getSourcePath(path_in_suite))
        if is_multi_file_test(source_path):
            for t in self.getTestsForPath(
                testSuite, path_in_suite, litConfig, localConfig
            ):
                yield t
            return
        for entry in source_path.iterdir():
            if (
                entry.is_file()
                and entry.suffix in (".cpp", ".cc", ".c")
                and not entry.name.endswith(IGNORED_SUFFIXES)
                and is_runnable_test(entry)
            ):
                for t in self.getTestsForPath(
                    testSuite, path_in_suite + (entry.name,), litConfig, localConfig
                ):
                    yield t


def read_rust_version():
    toolchain_path = os.path.join(
        os.path.dirname(__file__), "../../../../cmake/rust-toolchain.cmake"
    )
    with open(toolchain_path, "r") as f:
        for line in f:
            m = re.match(r'set\(RUST_STABLE_VERSION\s+"([^"]+)', line)
            if m:
                return m.group(1)
    raise Exception("could not find rust version in " + toolchain_path)


def only_tolerated_errors(err):
    errors = [
        line
        for line in RE_RUSTC_ERROR.findall(err)
        if not line.startswith("error: aborting due to")
    ]
    return bool(errors) and all(
        any(pattern.match(line) for pattern in TOLERATED_RUSTC_ERRORS)
        for line in errors
    )


def get_temp_dir():
    shm = Path("/dev/shm")
    if shm.exists() and os.access(shm, os.W_OK):
        return shm / "cpp2rust-tests"
    return Path("/tmp/cpp2rust-tests")


def is_multi_file_test(p):
    return p.is_dir() and (p / "CMakeLists.txt").exists()


def load_source_text(cc_input):
    if cc_input.is_dir():
        expectations_path = cc_input / "test.expectations"
        if not expectations_path.exists():
            return ""
        return "// " + expectations_path.read_text()
    return cc_input.read_text(errors="replace")


def collect_support_sources(cc_input, config):
    support_sources = getattr(config, "support_sources", None)
    if not support_sources or cc_input.is_dir():
        return []
    support_dir = Path(config.support_dir)
    seen = set()
    pending = [cc_input]
    while pending:
        current = pending.pop()
        for header in RE_INCLUDE.findall(current.read_text(errors="replace")):
            for path in (current.parent / header, support_dir / header):
                if path.is_file():
                    path = path.resolve()
                    if path not in seen:
                        seen.add(path)
                        pending.append(path)
                    break
    return [
        support_dir / src
        for hdr, src in support_sources.items()
        if (support_dir / hdr).resolve() in seen
    ]


def cmake_list(items):
    return " ".join(f'"{i}"' for i in items)


def read_stdin_file(cc_input, is_multi):
    # Interactive libstdc++ tests read stdin from a sibling <name>.in file
    stdin_file = cc_input.with_suffix(".in")
    if is_multi or not stdin_file.is_file():
        return None
    return stdin_file.read_text()


def setup_build_dir(tmp_dir, cc_input, force_std=None):
    build_dir = (tmp_dir / "cmake-build").resolve()
    build_dir.mkdir(parents=True, exist_ok=True)
    cmd = [
        "cmake",
        "-S",
        str(cc_input),
        "-B",
        str(build_dir),
        "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
    ]
    if force_std:
        cmd += [
            f"-DCMAKE_CXX_STANDARD={force_std}",
            "-DCMAKE_CXX_STANDARD_REQUIRED=ON",
            "-DCMAKE_CXX_EXTENSIONS=ON",
        ]
    _, err, rc = lit.util.executeCommand(cmd)
    if rc != 0:
        return None, "cmake configure failed\n" + err
    return build_dir, None


def cpp2rust_command(cc_input, build_dir, model, rs_file, extra_cxxflags):
    if build_dir is not None:
        return [
            "./cpp2rust/cpp2rust",
            "-dir",
            str(build_dir),
            "-model",
            model,
            "-o",
            str(rs_file),
        ]
    cmd = [
        "./cpp2rust/cpp2rust",
        "-file",
        str(cc_input),
        "-model",
        model,
        "-o",
        str(rs_file),
        f"-cxxflags=-I{cc_input.parent}",
    ]
    for flag in extra_cxxflags:
        cmd.append(f"-cxxflags={flag}")
    return cmd


def parse_feature_list(regex, text):
    return [
        item.strip()
        for m in regex.finditer(text)
        for item in m.group(1).split(",")
        if item.strip()
    ]


def unsupported_reason(test):
    if test.config.unsupported:
        return "Test is unsupported by its directory configuration"
    rel = os.path.relpath(test.getFilePath(), test.suite.source_root)
    if rel in getattr(test.config, "unsupported_paths", ()):
        return "Test is unsupported by its directory configuration"
    text = load_source_text(Path(test.getFilePath()))
    if unsupported := dg_unsupported_reason(text, test.config.available_features):
        return unsupported
    test.requires = parse_feature_list(RE_REQUIRES, text)
    test.unsupported = parse_feature_list(RE_UNSUPPORTED, text)
    if missing := test.getMissingRequiredFeatures():
        return "Test requires the following unavailable features: " + ", ".join(missing)
    if unsupported := test.getUnsupportedFeatures():
        return "Test does not support the following features: " + ", ".join(unsupported)
    return None


def is_runnable_test(path):
    return dg_do_kind(path.read_text(errors="replace")) == "run"


def dg_do_kind(text):
    dg_do = RE_DG_DO.search(text)
    return dg_do.group(1) if dg_do else "run"


def dg_unsupported_reason(text, features):
    dg_do = RE_DG_DO.search(text)
    selector = dg_do.group(2) if dg_do else ""
    if selector:
        m = RE_DG_TARGET_STD.fullmatch(selector)
        if m is None:
            return f"Unsupported dg-do selector: {selector}"
        if f"c++{m.group(1)}" not in features:
            return f"Test requires c++{m.group(1)}"
    if m := RE_DG_SKIP.search(text):
        return f"Unsupported directive: {m.group(1)}"
    return None


def dg_std(text, default_std):
    def year(std):
        return int(std) + (1900 if int(std) >= 98 else 2000)

    dg_do = RE_DG_DO.search(text)
    m = RE_DG_TARGET_STD.fullmatch(dg_do.group(2)) if dg_do else None
    if m and year(m.group(1)) > year(default_std):
        return m.group(1)
    return default_std


def parse_additional_compile_flags(text, features):
    flags = []
    for m in RE_DG_OPTIONS.finditer(text):
        selector = m.group(2)
        if selector is not None and not DG_TARGET_SELECTORS.get(selector, False):
            continue
        args = iter(shlex.split(m.group(1)))
        flags.extend(a + next(args) if a == "-include" else a for a in args)
    for m in RE_ADDITIONAL_COMPILE_FLAGS.finditer(text):
        guard = m.group(1)
        if guard is None or guard in features:
            flags.extend(shlex.split(m.group(2)))
    return flags


def get_expected_file(filepath, model, fname):
    return filepath / "out" / model / f"{fname}.rs"


def update_expected(generated, expected_path):
    expected_path.parent.mkdir(parents=True, exist_ok=True)
    expected_path.write_text(generated)
